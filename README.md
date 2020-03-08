# Language Rust
# Introduction
Rust est un nouveau langage de programmation open source créé par Mozilla et une communauté de volontaires, conçu pour aider les développeurs à concevoir des applications ultra-rapides et sécurisées qui utilisent avantageusement les puissantes fonctionnalités offertes par les processeurs multi-cœurs modernes. Il permet d'éviter les erreurs de segmentation et assure la sécurité des threads, le tout avec une syntaxe facile à assimiler.
# Bases du language
## Premier programme
Pour créer un premier programme, nous allons créer un fichier main.rs (rs est l'extention des fichiers Rust) qui va contenir le code: 

```markdown
fn main() {
    println!("Bonjour tous le monde!");
}
Ce petit programme imprime le texte "Bonjour tous le monde" à l'écran.
```
## Les variables
En Rust, toutes les variables locales sont déclarées par let et sont des constantes (on ne peut pas changer sa valeur). Il faut ajouter le déclarateur mut, pour pouvoir les déclarer comme variable mutable.

### Exemple variable constante
```markdown
fn main() {
    let x = 0;
    x = 4; // Affiche un message d'erreur ==> error[E0384]: cannot assign twice to immutable variable `x`
    println!("La valeur de x est : {}", x);
}
```

### Exemple variable mutable
```markdown
fn main() {
    let mut x = 0;
    x = 4; // ok
    println!("La valeur de x est : {}", x);// affiche La valeur de x est : 4
}
```
## Les types des variables
Chaque valeur dans Rust est associée à un type de données comme les autres languages de programmation. Donc on trouve: des entiers, des booleans, des flottants, des chaines de caracères(Strings)...Mais la déclaration d'une variable avec un type est différentes en Rust par rapport aux autres languages.

### Les entiers
Un nombre entier est un nombre que l'on peut écrire sans virgule. En Rust les types d'entiers signés commencent par i et non signés par u.
Exemple des entiers:
 ```markdown
 
 let i : i8 = 0; // Entier signé qui occupe 8 bits d'espace
 let i : i32 = 0; //Entier signé qui occupe 32 bits d'espace
let i = 7i18;//Entier signé qui occupe 18 bits d'espace

 ```
### Les booleans
un type booléen dans Rust a deux valeurs possibles: true et false. Le type booléen dans Rust est spécifié en utilisant bool. 
exemple:

 ```markdown
 
fn main() {
    let a: bool = true; // type explicite en utilisant bool
    let b = false; // le type est boolean implicitement avec la valeur false
}
 ```
### Les flottants
Un flottant est un nombre avec la virgule. Les types à virgule flottante de Rust sont f32 et f64, qui sont respectivement de 32 bits et 64 bits. Le type par défaut est f64.

 ```markdown
fn main() {

    let x = 0.0; // Variable de type float 64 bit: type inplicite
    let xx: f64 = 0.0; // Variable de type float 64 bit: type explicite 
}
 ```
  
### Char 
Le type de caractère de Rust est le type alphabétique le plus primitif de la langue, et le code suivant montre une façon de l'utiliser:

```markdown
fn main() {
    let a = 'a';// caractère 'a'
    let b = 'B';
}
```
## Types composés

Les types composés peuvent regrouper plusieurs valeurs en un seul type

## Tuple
Un tuple est un moyen général de regrouper en un seul type un certain nombre de valeurs avec une des types différents en un seul type composé. Les tuples ont une longueur fixe.

Nous créons un tuple en écrivant une liste de valeurs séparées par des virgules entre parenthèses. Chaque position dans le tuple a un type, et les types des différentes valeurs dans le tuple n'ont pas à être les mêmes.
Exemple:
```markdown

let t = (0, 5.0, false, "Rust");

```

## Array
