
#[macro_export]
macro_rules! expect {
    ($this:expr, $typ:pat) => {
        {
            match $this.tokens.next() {
                Some(t) => {
                    match t.typ {
                        $typ{..} => t,
                        _ => {
                            $this.errors.0 += 1;
                            error!(&t.loc, "Expected {}, found {}", stringify!($typ), t.typ.to_string());
                        }
                    }
                },
                None => {
                    $this.errors.0 += 1;
                    error!("Expected {}, found Nothing", stringify!($typ));
                },
            }
        }
    };
    
}

#[macro_export]
macro_rules! error_expect {
    ($this:expr, $typ:pat) => {
        {
            match $this.tokens.next() {
                Some(t) => {
                    match t.typ {
                        $typ => t,
                        _ => {
                            $this.errors.0 += 1;
                            error!(&t.loc, "Expected {}, found {}", stringify!($typ), t.typ.to_string());
                            bail!("Error");
                        }
                    }
                },
                None => {
                    $this.errors.0 += 1;
                    error!("Expected {}, found Nothing", stringify!($typ));
                    bail!("Error");
                },
            }
        }
    };
    
}