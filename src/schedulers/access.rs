use std::{
    any::TypeId,
    collections::{hash_map, HashMap},
};

use crate::components::{components::Component, resources::Resource};

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
        if let (AccessAction::Read(_), AccessAction::Read(_)) = (self, other) {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Access {
    data: HashMap<AccessType, AccessAction>,
}

impl IntoIterator for Access {
    type Item = (AccessType, AccessAction);

    type IntoIter = hash_map::IntoIter<AccessType, AccessAction>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl FromIterator<(AccessType, AccessAction)> for Access {
    fn from_iter<T: IntoIterator<Item = (AccessType, AccessAction)>>(iter: T) -> Self {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

impl Access {
    /// returns the iter of the accessed values
    pub fn iter(&self) -> std::collections::hash_map::Iter<AccessType, AccessAction> {
        self.data.iter()
    }
    /// returns the state of a single field
    pub fn at(&self, ty: &AccessType) -> Option<&AccessAction> {
        self.data.get(ty)
    }
    pub fn at_mut(&mut self, ty: &AccessType) -> Option<&mut AccessAction> {
        self.data.get_mut(ty)
    }

    /// checks whether other access value is compatible with current
    pub fn is_compatible(&self, other: &Self) -> bool {
        for (ty, a) in self.iter() {
            if let Some(b) = other.at(ty) {
                if !b.is_compatible(a) {
                    return false;
                }
            }
        }
        true
    }

    /// trys to combine 2 access value to save the current state.
    pub fn try_combine(&mut self, other: &Self) -> Result<(), ()> {
        let mut candidate = self.clone();
        for (id, other) in other.iter() {
            if let Some(curr) = candidate.at_mut(id) {
                match (other, curr) {
                    (AccessAction::Read(a), AccessAction::Read(b)) => {
                        *b += a;
                    }
                    _ => {
                        return Err(());
                    }
                }
            } else {
                candidate.insert(*id, *other);
            }
        }
        *self = candidate;
        Ok(())
    }
    fn insert(&mut self, id: AccessType, other: AccessAction) -> Option<AccessAction> {
        self.data.insert(id, other)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn remove(&mut self, other: &Access) {
        for (i, t) in other.iter() {
            if let (Some(AccessAction::Read(a)), AccessAction::Read(b)) = (self.at_mut(i), t) {
                if *a >= *b {
                    *a -= b;
                }
                if *a == 0 {
                    self.data.remove(i);
                }
            } else if let AccessAction::Write = t {
                self.data.remove(i);
            }
        }
    }
}

mod tests {

    #[test]
    fn test_access() {
        use crate::schedulers::access::{Access, AccessAction};
        use crate::{components::components::Component, schedulers::access::AccessType};

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
