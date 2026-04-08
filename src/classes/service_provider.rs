use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

type ServiceFactory = Box<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync>;

pub struct ServiceProvider {
    scoped_factories: HashMap<TypeId, ServiceFactory>,
    singleton_services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl ServiceProvider {
    pub fn new() -> Self {
        Self {
            scoped_factories: HashMap::new(),
            singleton_services: HashMap::new(),
        }
    }

    pub fn add_scope<T, F>(&mut self, factory: F)
    where
        T: Send + Sync + 'static,
        F: Fn() -> T + Send + Sync + 'static,
    {
        self.scoped_factories
            .insert(TypeId::of::<T>(), Box::new(move || Arc::new(factory())));
    }

    pub fn add_singleton<T>(&mut self, service: T)
    where
        T: Send + Sync + 'static,
    {
        self.singleton_services
            .insert(TypeId::of::<T>(), Arc::new(service));
    }

    pub fn get_service<T>(&self) -> Option<Arc<T>>
    where
        T: Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();

        if let Some(singleton) = self.singleton_services.get(&type_id) {
            return Arc::downcast::<T>(Arc::clone(singleton)).ok();
        }

        let factory = self.scoped_factories.get(&type_id)?;
        Arc::downcast::<T>((factory)()).ok()
    }
}

impl Default for ServiceProvider {
    fn default() -> Self {
        Self::new()
    }
}