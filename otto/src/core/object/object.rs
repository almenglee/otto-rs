use super::id::ID;

/// Shared object metadata.
///
/// Intended for composition-based object hierarchies.
pub struct ObjectData {
    id: Option<ID>,
    name: String,
}

/// Common object interface.
///
/// Provides access to object identity and metadata
/// through an embedded [`ObjectData`].
pub trait Object {
    fn object(&self) -> &ObjectData;
    fn object_mut(&mut self) -> &mut ObjectData;

    #[inline]
    fn id(&self) -> Option<ID> {
        self.object().id
    }

    #[inline]
    fn name(&self) -> &str {
        &self.object().name
    }


    #[inline]
    fn set_id(&mut self, id: ID) {
        self.object_mut().id = Some(id);
    }
}

pub struct Entry {
    object: Box<dyn Object>,
    meta: ObjectData,
}

pub struct Registry {
    entries: Vec<Entry>,
}