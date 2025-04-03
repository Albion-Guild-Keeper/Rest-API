use serde::{Deserialize, Serialize};
use std::fmt;

// Definisce una struttura chiamata SurrealInt che contiene un singolo campo pubblico di tipo i64.
// L'uso di pub permette di accedere al campo dall'esterno della struttura.
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct SurrealInt(pub i64);

// Rimuovi l'implementazione di FromParam se non è necessaria
// Implementazioni per conversioni
impl From<SurrealInt> for i64 {
    fn from(val: SurrealInt) -> Self {
        val.0
    }
}

impl From<i64> for SurrealInt {
    fn from(val: i64) -> Self {
        SurrealInt(val)
    }
}

impl fmt::Display for SurrealInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Spiegazione di 'de:
// 'de è un lifetime parameter (parametro di durata). In Rust, i lifetime sono un modo per
// garantire che i riferimenti siano validi per un certo periodo di tempo.
// In questo caso, 'de indica che il lifetime del deserializzatore D è lo stesso del lifetime
// dei dati deserializzati. Questo è importante per garantire che i dati deserializzati
// non sopravvivano al deserializzatore stesso, evitando problemi di dangling references
// (riferimenti pendenti).