# Gestionnaire de photos

Gestionnaire de photos est une application, développé en Rust, s'éxecute en ligne du commande. Elle permet de gérér les metadatas(EXEF, XMP ou IPTC) des photos selon des déffirents critères.
# Fonctionalités

L'application propose à l'utilisateur de séléctionner un ensemble de photos selon plusieurs critères à savoir:
* Le nom de la photo
* La date de prise de vue
* La localisation à partier des trois paramètres : longitude, latitude et altitude.
* Le nom de l'appareil photo ou la camera
* La résolution de la photo
* tags qui sont des expressions ajoutées par l'utilisateur.

Après que l'utilisateur choisi le mode de sélection, l'application demande de saisir les informations nécessaires pour chaque mode. si l'utilisateur choisi :
* La sélection par le nom : il faudra saisir un nom ou une partie d'un nom pour sélectionner l'ensemble des photos qui ont le même nom ou qui contient le même mots.
* La sélection par date : il faudra saisir une date, en respectant le format 0000:00:00, pour séléctionner les photos qui ont été prit dans le même jour.
* La sélection par localisation : l'utilisateur doit saisir successivement la longitude, la latitude , et l'altitude (chaque saisi est suivi de ENTER).
* La selection par l'appareil photo : l'application demande à l'utilisateur de saisir le nom de l'appariel photo.
* La sélection par la résolution : il faudra saisir la résolution X et la résolution Y.
* La sélection par tags : il faudra saisir une expression pour sélectionner les photons qui contient le même mots-clés.
Après la selection, l'application affiche les photos séléctionnées avec certains informations pour chaque photo (le nom, le type, l'appariel photo, la date, la résolution et les tags). 
Dans la sélection, l'application propose à l'utilisateur d'ajouter ou supprimer une ou plusieurs expressions (des commentaires). Le résultas sera stocké dans les métadonnées des photos selectionnées. Les expressions saisies doivent être séparées par le caractère "/".

# Construction de projet
Le projet est construit avec le gestionnaire des paquets de Rust : **Cargo**

```
imagemanager
└───images
└───src
│   │   main.rs
│   │   lib.rs 
└───tests
    │   test.rs
└─── Cargo.toml
```
* **images** : est un répértoire qui contient les photos à gérer.
* **src/main.rs** : contient la fonction principale main.
* **src/lib.rs** : le code source de l'application (structs, les méthodes...)
* **tests/test.rs** : les tests unitaire de l'application
* **cargo.toml** : décrit le projet et il contient les dépendances utilisées.

## Compilation et l'execution

Pour compiler et lancer l'exécutable le projet, tout d'abord, il faudra se déplacer dans le répértoire du projet **imagemanager**.Ensuite utiliser la commande run :

```
cargo run
```
Les testes unitaires de l'application se lance avec la commande: 

```
cargo test
```
# Amélioration
L’ensemble du cahier des charges que nous nous étions fixé semble être respecté. Nous aurions
aimé ajouter plus de fonctionnalités notamment la sélection par type(JPG,PNG...) et sélection par la supportation des défférents tags (Exef, Xmp, Iptc). Ainsi que permetttre l'utilisateur de modifier certaines méta-données.


Pour savoir plus sur la conception et les sénarios du projet, veuillez voir le documenent **conception_sebario.md**


