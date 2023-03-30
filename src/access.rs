use std::{any::TypeId, collections::{HashMap, hash_map}};

#[derive(Clone, Copy)]
pub enum AccessType {
    Read,
    Write,
}

impl AccessType {
    pub fn is_compatible(&self, other: &Self) -> bool {
        if let (AccessType::Read, AccessType::Read) = (self, other) {
            true
        } else {
            false
        }
    }
}

#[derive(Default)]
pub struct Access {
    data: HashMap<TypeId, AccessType>,
}

impl IntoIterator for Access {
    type Item = (TypeId,AccessType);

    type IntoIter = hash_map::IntoIter<TypeId,AccessType>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
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
    pub fn iter(&self) -> std::collections::hash_map::Iter<TypeId, AccessType> {
        self.data.iter()
    }
    pub fn at(&self, ty: &TypeId) -> Option<&AccessType> {
        self.data.get(ty)
    }
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
    pub fn try_combine(&mut self, other: &Self) -> Result<(),()> {
        if self.is_compatible(other) {
            for (ty, a) in other.iter() {
                self.data.entry(ty.to_owned()).or_insert(a.to_owned());
            }

            Ok(())
        } else {
            Err(())
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
                (TypeId::of::<T>(), AccessType::Read)
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
