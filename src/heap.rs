use std::{
    any::{type_name, Any},
    collections::HashMap,
    fmt::{self, Debug},
    marker::PhantomData,
    mem,
};

pub trait Object {
    fn size(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Allocation {
    size: usize,
    obj: Box<dyn Object>,
}

pub struct Heap {
    bytes_allocated: usize,
    objects: Vec<Option<Allocation>>,
    free_slots: Vec<usize>,
    strings: HashMap<String, Ref<String>>,
}

pub struct Ref<T: Object> {
    index: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Object> Copy for Ref<T> {}

impl<T: Object> Clone for Ref<T> {
    #[inline]
    fn clone(&self) -> Ref<T> {
        *self
    }
}

impl<T: Object> Debug for Ref<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let full_name = type_name::<T>();
        full_name.split("::").last().unwrap();
        write!(f, "ref({}:{})", self.index, full_name)
    }
}

impl Heap {
    pub fn new() -> Self {
        Self {
            bytes_allocated: 0,
            objects: Vec::new(),
            free_slots: Vec::new(),
            strings: HashMap::new(),
        }
    }

    pub fn alloc<T: Object + 'static + Debug>(&mut self, object: T) -> Ref<T> {
        let size = object.size() + mem::size_of::<Allocation>();
        self.bytes_allocated += size;
        let entry = Allocation {
            size,
            obj: Box::new(object),
        };

        let index = match self.free_slots.pop() {
            Some(i) => {
                self.objects[i] = Some(entry);
                i
            }
            None => {
                self.objects.push(Some(entry));
                self.objects.len() - 1
            }
        };

        Ref {
            index,
            _marker: PhantomData,
        }
    }

    pub fn alloc_string(&mut self, name: String) -> Ref<String> {
        if let Some(&value) = self.strings.get(&name) {
            value
        } else {
            let reference = self.alloc(name.clone());
            self.strings.insert(name, reference);
            reference
        }
    }

    pub fn deref<T: Object + 'static>(&self, reference: Ref<T>) -> &T {
        self.objects[reference.index]
            .as_ref()
            .unwrap()
            .obj
            .as_any()
            .downcast_ref()
            .unwrap_or_else(|| panic!("Reference {} not found", reference.index))
    }

    pub fn deref_mut<T: Object + 'static>(&mut self, reference: Ref<T>) -> &mut T {
        self.objects[reference.index]
            .as_mut()
            .unwrap()
            .obj
            .as_any_mut()
            .downcast_mut()
            .unwrap_or_else(|| panic!("Reference {} not found", reference.index))
    }
}
