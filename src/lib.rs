use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    // dummy is needed to avoid unused parameter error during compilation
    dummy: ::std::marker::PhantomData<T>,

    valeur: i32, //La valeur du noeud de la liste chainée est un entier de 32 bits (i32).

    //L'adresse du nœud suivant est contenu dans cette variable, sauf que dans RUST au moment de
    //la compilation nous devons déclarer les tailles exacts de chaque variable par exemple.
    //Mais comme nous savons pas encore la taille du prochain nœud alors on pourra pas le stocker
    //dans le STACK, il faudra alors allouer l'espace dans le HEAP et pour cela il faudra utiliser
    //Option<Box. Ça fonctionne mais c'est ce qu'on appelle unsafe code dans RUST est donc il
    //faudra penser à dé allouer l'espace pris par soi meme.
    suivant: Option<Box<SimpleLinkedList<T>>>,
}

pub struct PREMIER<T> {
    // Le head correspond à la valeur du tout premier nœud dans la liste.
    head: Option<Box<SimpleLinkedList<T>>>,
}

// Ici on crée une liste chainée avec un premier élément vide (None).
impl<T> PREMIER<T> {
    pub fn new() -> Self {
        PREMIER{head: None}
    }

    //Ici la fonction doit juste nous dire si la liste est vide (true) ou pas (false).
    pub fn is_empty(&self) -> bool {
        if !&self.is_empty(){
            true
            }
        false
    }
    }

    // Ici la fonction doit juste nous retourner la taille de la liste.
    pub fn len(&self) -> usize {
        let mut compteur: u32 = 0; //Compteur qui aura la taille de la liste entière.
        let mut position_actuelle = &self.head; // Ce pointeur va parcourir la liste à partir du head.

        while position_actuelle.is_ok(){
            position_actuelle = &self.suivant;
            compteur++;
        }
        compteur
    }

    pub fn push(&mut self, _element: T) {
        unimplemented!()
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        unimplemented!()
    }
}
