use std::thread;
use std::sync::{Mutex, Arc};

struct Filosofo {
    nombre: String,
    izquierda: usize,
    derecha: usize,
}

impl Filosofo {
    fn new(nombre: &str, izquierda: usize, derecha: usize) -> Filosofo {
        Filosofo {
            nombre: nombre.to_string(),
            izquierda: izquierda,
            derecha: derecha,
        }
    }

    fn comer(&self, mesa: &Mesa) {
        let _izquierda = mesa.tenedores[self.izquierda].lock().unwrap();
        let _derecha = mesa.tenedores[self.derecha].lock().unwrap();

        println!("{} esta comiendo.", self.nombre);

        thread::sleep_ms(1000);

        println!("{} ha finalizado de comer.", self.nombre);
    }
}

struct Mesa {
    tenedores: Vec<Mutex<()>>,
}

fn main() {
    let mesa = Arc::new(Mesa { tenedores: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let filosofos = vec![
        Filosofo::new("Judith Butler", 0, 1),
        Filosofo::new("Gilles Deleuze", 1, 2),
        Filosofo::new("Karl Marx", 2, 3),
        Filosofo::new("Emma Goldman", 3, 4),
        Filosofo::new("Michel Foucault", 0, 4),
    ];

    let handles: Vec<_> = filosofos.into_iter().map(|f| {
        let mesa = mesa.clone();

        thread::spawn(move || {
            f.comer(&mesa);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
