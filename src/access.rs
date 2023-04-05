use std::{
    any::TypeId,
    collections::{hash_map, HashMap},
};

#[derive(Debug, Clone, Copy)]
pub enum AccessType {
    Read(usize),
    Write,
}

impl AccessType {
    pub fn is_compatible(&self, other: &Self) -> bool {
        if let (AccessType::Read(_), AccessType::Read(_)) = (self, other) {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Access {
    data: HashMap<TypeId, AccessType>,
}

impl IntoIterator for Access {
    type Item = (TypeId, AccessType);

    type IntoIter = hash_map::IntoIter<TypeId, AccessType>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl FromIterator<(TypeId, AccessType)> for Access {
    fn from_iter<T: IntoIterator<Item = (TypeId, AccessType)>>(iter: T) -> Self {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

impl Access {
    /// returns the iter of the accessed values
    pub fn iter(&self) -> std::collections::hash_map::Iter<TypeId, AccessType> {
        self.data.iter()
    }
    /// returns the state of a single field
    pub fn at(&self, ty: &TypeId) -> Option<&AccessType> {
        self.data.get(ty)
    }
    pub fn at_mut(&mut self, ty: &TypeId) -> Option<&mut AccessType> {
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
                    (AccessType::Read(a), AccessType::Read(b)) => {
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
    fn insert(&mut self, id: TypeId, other: AccessType) -> Option<AccessType> {
        self.data.insert(id, other)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn remove(&mut self, other: &Access) {
        for (i, t) in other.iter() {
            if let (Some(AccessType::Read(a)), AccessType::Read(b)) = (self.at_mut(i), t) {
                if *a >= *b {
                    *a -= b;
                }
                if *a == 0 {
                    self.data.remove(i);
                }
            } else if let AccessType::Write = t {
                self.data.remove(i);
            }
        }
    }
}

mod tests {

    #[test]
    fn test_access() {
        use std::any::TypeId;

        use crate::access::{Access, AccessType};
        trait AccessAccessor {
            fn read() -> (TypeId, AccessType);
            fn write() -> (TypeId, AccessType);
        }

        impl<T: 'static> AccessAccessor for T {
            fn read() -> (TypeId, AccessType) {
                (TypeId::of::<T>(), AccessType::Read(1))
            }

            fn write() -> (TypeId, AccessType) {
                (TypeId::of::<T>(), AccessType::Write)
            }
        }

        struct A;
        struct B;
        struct C;

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
