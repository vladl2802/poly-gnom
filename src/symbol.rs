use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Debug, Display},
    hash::Hash,
    ops::Deref,
    rc::Rc,
};

#[derive(Clone)]
pub struct SymbolsProvider<Types> {
    symbols: Rc<RefCell<SymbolsProviderData<Types>>>,
}

#[derive(Clone)]
pub struct SymbolInfo<Types> {
    pub label: String,
    pub associated_type: Option<Types>,
}

impl<Types> SymbolInfo<Types> {
    pub fn new(label: &str, associated_type: Option<Types>) -> Self {
        SymbolInfo {
            label: label.to_owned(),
            associated_type,
        }
    }

    pub fn new_typed(label: &str, associated_type: Types) -> Self {
        Self::new(label, Some(associated_type))
    }
}

pub struct Symbol<Types> {
    info: Rc<SymbolInfo<Types>>,
}

impl<Types> Debug for Symbol<Types>
where
    Types: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {:?} | ", self.label)?;
        match &self.associated_type {
            Some(associated_type) => write!(f, "{:?}", associated_type),
            None => write!(f, "None"),
        }?;
        write!(f, " )")?;
        Ok(())
    }
}

impl<Types> Display for Symbol<Types>
where
    Types: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl<Types> Symbol<Types> {
    fn new(info: SymbolInfo<Types>) -> Symbol<Types> {
        Symbol {
            info: Rc::new(info),
        }
    }
}

impl<Types> Clone for Symbol<Types> {
    fn clone(&self) -> Self {
        Self {
            info: self.info.clone(),
        }
    }
}

impl<Types> PartialEq for Symbol<Types> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.info, &other.info)
    }
}

impl<Types> Hash for Symbol<Types> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.info).hash(state);
    }
}

impl<Types> Deref for Symbol<Types> {
    type Target = SymbolInfo<Types>;

    fn deref(&self) -> &Self::Target {
        self.info.as_ref()
    }
}

type SymbolsProviderData<Types> = HashMap<String, Symbol<Types>>;
// label is coppied two times, but I don't find easy way to avoid it (except wrapping it with yet another rc).
// I want to have label inside SymbolInfo because symbol without name is just stupid,
// but I also want to preserve fast lookups for some label to find its info.
// That probably can be done with HashSet and private equality operator for Info, that will compare only by label
// but thats not all because we need some kind of dummy associated_type in order to perform get
// in the end this will clutter user interface of Symbol and SymbolInfo, so I avoid that

impl<Types> SymbolsProvider<Types> {
    pub fn empty() -> Self {
        SymbolsProvider {
            symbols: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn add(&self, info: SymbolInfo<Types>) -> Symbol<Types> {
        let mut symbols = self.symbols.borrow_mut();
        symbols
            .entry(info.label.clone())
            .or_insert(Symbol::new(info))
            .clone()
    }

    pub fn get(&self, label: &str) -> Option<Symbol<Types>> {
        self.symbols.borrow().get(label).cloned()
    }
}
