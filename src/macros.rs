macro_rules! gen_string_method {
    ($field:ident) => {
        pub fn $field<S>(mut self, $field: S) -> Self
        where
            S: Into<String>,
        {
            self.spec.$field = $field.into();
            self
        }
    };
    ($field:ident, $($fields:ident),+) => {
        gen_string_method!($field);
        gen_string_method!($($fields),+);
    }
}

macro_rules! gen_bool_method {
    ($field:ident) => {
        pub fn $field(mut self, $field: bool) -> Self
        {
            self.spec.$field = $field;
            self
        }
    };
    ($field:ident, $($fields:ident),+) => {
        gen_string_method!($field);
        gen_string_method!($($fields),+);
    }
}

macro_rules! gen_option_method {
    ($field:ident) => {
        pub fn $field<S>(mut self, $field: S) -> Self
        where
            S: Into<String>,
        {
            self.spec.$field = Some($field.into());
            self
        }
    };
    ($field:ident, $($fields:ident),+) => {
        gen_option_method!($field);
        gen_option_method!($($fields),+);
    }
}

macro_rules! gen_vec_method {
    ($field:ident) => {
        paste! {
        pub fn [<add_ $field _entries>]<I, S>(mut self, entries: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: Into<String>,
        {
            entries.into_iter().for_each(|entry| self.spec.$field.push(entry.into()));
            self
        }
        }
    };
    ($field:ident, $($fields:ident),+) => {
        gen_vec_method!($field);
        gen_vec_method!($($fields),+);
    }
}
