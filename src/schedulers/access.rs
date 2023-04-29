use std::{
    any::TypeId,
    collections::{hash_map, HashMap},
};

use lazy_static::lazy_static;

use crate::components::{component::Component, resources::Resource};

#[derive(Debug, Clone, Copy)]
pub enum AccessAction {
    Read(usize),
    Write,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AccessType {
    Component(TypeId),
    Resource(TypeId),
}

impl AccessType {
    pub fn component<T: Component + 'static>() -> Self {
        Self::Component(TypeId::of::<T>())
    }

    pub fn resource<T: Resource + 'static>() -> Self {
        Self::Resource(TypeId::of::<T>())
    }
}

impl AccessAction {
    pub fn is_compatible(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (AccessAction::Read(_), AccessAction::Read(_))
        )
    }
}

#[derive(Debug, Clone)]
pub enum AccessState {
    With(HashMap<AccessType, AccessAction>),
    All,
}
impl AccessState {
    pub fn get_map(&self) -> Result<&HashMap<AccessType, AccessAction>, AccessIsAllErr> {
        match self {
            Self::With(data) => Ok(data),
            Self::All => Err(AccessIsAllErr),
        }
    }

    pub fn get_map_mut(
        &mut self,
    ) -> Result<&mut HashMap<AccessType, AccessAction>, AccessIsAllErr> {
        match self {
            Self::With(data) => Ok(data),
            Self::All => Err(AccessIsAllErr),
        }
    }

    fn empty() -> AccessState {
        Self::With(Default::default())
    }
}

impl Default for AccessState {
    fn default() -> Self {
        AccessState::With(HashMap::default())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Access {
    data: AccessState,
}

impl IntoIterator for Access {
    type Item = (AccessType, AccessAction);

    type IntoIter = hash_map::IntoIter<AccessType, AccessAction>;

    fn into_iter(self) -> Self::IntoIter {
        match self.data {
            AccessState::With(data) => data.into_iter(),
            AccessState::All => EMPTY_ACCESS.clone().into_iter(),
        }
    }
}

impl FromIterator<(AccessType, AccessAction)> for Access {
    fn from_iter<T: IntoIterator<Item = (AccessType, AccessAction)>>(iter: T) -> Self {
        Self {
            data: AccessState::With(iter.into_iter().collect()),
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct AccessIsAllErr;

lazy_static! {
    static ref EMPTY_ACCESS: Access = Access::empty();
}

impl Access {
    /// returns the iter of the accessed values
    pub fn iter(&self) -> std::collections::hash_map::Iter<AccessType, AccessAction> {
        match &self.data {
            AccessState::With(data) => data.iter(),
            AccessState::All => EMPTY_ACCESS.iter(),
        }
    }
    /// returns the state of a single field
    pub fn at(&self, ty: &AccessType) -> Result<Option<&AccessAction>, AccessIsAllErr> {
        Ok(self.data.get_map()?.get(ty))
    }
    pub fn at_mut(&mut self, ty: &AccessType) -> Result<Option<&mut AccessAction>, AccessIsAllErr> {
        Ok(self.data.get_map_mut()?.get_mut(ty))
    }

    pub fn get_map(&self) -> Result<&HashMap<AccessType, AccessAction>, AccessIsAllErr> {
        self.data.get_map()
    }

    pub fn get_map_mut(
        &mut self,
    ) -> Result<&mut HashMap<AccessType, AccessAction>, AccessIsAllErr> {
        self.data.get_map_mut()
    }
    /// checks whether other access value is compatible with current
    pub fn is_compatible(&self, other: &Self) -> bool {
        match (self.get_map(), other.get_map()) {
            (Ok(map_a), Ok(map_b)) => {
                for (ty, a) in map_a.iter() {
                    if let Some(b) = map_b.get(ty) {
                        if !b.is_compatible(a) {
                            return false;
                        }
                    }
                }
                true
            }
            (Ok(a), Err(_)) => a.is_empty(),
            _ => false,
        }
    }

    /// trys to combine 2 access value to save the current state.
    pub fn try_combine(&mut self, other: &Self) -> Result<(), AccessIsAllErr> {
        let candidate = match (self.get_map(), other.get_map()) {
            (Err(_), Err(_)) => Some(Self::empty()),
            (Ok(map_a), Ok(map_b)) => {
                let mut candidate = Access {
                    data: AccessState::With(map_a.clone()),
                };
                for (id, other) in map_b.iter() {
                    if let Some(curr) = candidate.at_mut(id).expect("this should not crash") {
                        match (other, curr) {
                            (AccessAction::Read(a), AccessAction::Read(b)) => {
                                *b += a;
                            }
                            _ => {
                                return Err(AccessIsAllErr);
                            }
                        }
                    } else {
                        candidate
                            .insert(*id, *other)
                            .expect("this should not crash");
                    }
                }
                Some(candidate)
            }
            (Ok(map_a), Err(_)) => {
                if map_a.is_empty() {
                    Some(Access::all())
                } else {
                    None
                }
            }

            _ => None,
        };
        match candidate {
            Some(a) => {
                *self = a;
                Ok(())
            }
            None => Err(AccessIsAllErr),
        }
    }
    fn insert(
        &mut self,
        id: AccessType,
        other: AccessAction,
    ) -> Result<Option<AccessAction>, AccessIsAllErr> {
        Ok(self.data.get_map_mut()?.insert(id, other))
    }

    pub fn clear(&mut self) {
        self.data = AccessState::empty();
    }

    pub fn remove(&mut self, other: &Access) {
        let a = self.data.get_map_mut();
        let b = other.data.get_map();
        match (a, b) {
            (Err(_), Err(_)) => self.data = AccessState::default(),
            (Ok(map_a), Ok(map_b)) => {
                for (i, t) in map_b.iter() {
                    if let (Some(AccessAction::Read(a)), AccessAction::Read(b)) =
                        (map_a.get_mut(i), t)
                    {
                        if *a >= *b {
                            *a -= b;
                        }
                        if *a == 0 {
                            map_a.remove(i);
                        }
                    } else if let AccessAction::Write = t {
                        map_a.remove(i);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn all() -> Access {
        Access {
            data: AccessState::All,
        }
    }

    pub fn empty() -> Access {
        Default::default()
    }
}

mod tests {

    #[test]
    fn test_access() {
        use crate::schedulers::access::{Access, AccessAction};
        use crate::{components::component::Component, schedulers::access::AccessType};

        trait AccessAccessor {
            fn read() -> (AccessType, AccessAction);
            fn write() -> (AccessType, AccessAction);
        }

        impl<T: Component + 'static> AccessAccessor for T {
            fn read() -> (AccessType, AccessAction) {
                (AccessType::component::<T>(), AccessAction::Read(1))
            }

            fn write() -> (AccessType, AccessAction) {
                (AccessType::component::<T>(), AccessAction::Write)
            }
        }

        struct A;
        impl Component for A {}
        struct B;
        impl Component for B {}
        struct C;
        impl Component for C {}

        let access1 = [A::read(), B::write(), C::read()]
            .into_iter()
            .collect::<Access>();
        let access2 = [A::read(), B::write(), C::read()]
            .into_iter()
            .collect::<Access>();

        let access3 = Access::from_iter([A::read(), B::write()]);
        let access4 = Access::from_iter([A::read(), C::write()]);

        assert!(!access1.is_compatible(&access2));
        assert!(access3.is_compatible(&access4));
        assert!(!access2.is_compatible(&access3));
    }
}
