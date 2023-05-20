/*
    Alla fie della riga 18 (non considerando lo scope della funzione) ci ritroveremo nello stack solo
    il riferimento alla slice del vettore v creato con l'operazione &v[..].

    Mentre viene invece memorizzato nell'heap di memoria, il vettore v poichè è un oggetto di dimensioni dinamiche
    insieme al suo contenuto di tipo PathCommand dove i tre creano ciascuno una nuova istanza alla struttura Point.
*/

struct Point {
    x: i16,
    y: i16,
}

enum PathCommand {
    Move(Point),
    Line(Point),
    Close,
}

fn main() {
    let mut v = Vec::<PathCommand>::new();
    v.push(PathCommand::Move(Point { x: 1, y: 1 }));
    v.push(PathCommand::Line(Point { x: 10, y: 20 }));
    v.push(PathCommand::Close);
    let _slice = &v[..];
}
