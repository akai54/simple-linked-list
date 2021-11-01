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

pub struct premier<T> {
head: Option<Box<SimpleLinkedList<T>>>, // Le head correspond à la valeur du tout premier nœud dans la liste.
}

impl<T> premier<T> {
    pub fn new() -> Self {
        premier{head: None}
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        unimplemented!()
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
