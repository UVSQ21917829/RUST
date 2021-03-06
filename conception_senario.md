# Conception technique
## Dépendences 
le projet utilise la bibliothèque externe rexiv2 (version 0.9.0). Cette dépendance est disponible sur crates.io.

```toml
[dependencies]
rexiv2="0.9.0"
```
Cette bibliothèque fournit un wrapper Rust de la bibliothèque gexiv2, qui est elle-même un wrapper basé sur GObject de la bibliothèque Exiv2, qui fournit un accès en lecture et en écriture aux métadonnées Exif, XMP et IPTC pour les fichiers multimédias dans différents formats.

Étant donné que la bibliothèque rexiv2 est lié à gexiv2 et transitivement à Exiv2, rexiv2 dépend évidemment d'eux (et de leurs dépendances). L'installation de ces bibliothèques sur le système est une condition préalable à l'utilisation de rexiv2 ou de tout logiciel construit sur celle-ci (le cas de notre projet).

## Réferences des APIs

[**gexiv2**](https://wiki.gnome.org/Projects/gexiv2)

[**exiv2**](https://www.exiv2.org/)

[**rexiv2**](https://github.com/felixc/rexiv2/blob/master/SETUP.md)

# Sénario
Dans le sénario, on va montrer le résultat de chaque fonctionnalités de l'application .
Apères le lancement de l'application, l'utilisateur doit choisir le répertoire des photos en entrant son chemin à fin de charger les photos à gérer.

![figure](choisir_rep.png)

Après avoir saisir un chemin valide, l'application propose à l'utilisateur de choisir un mode de séléction 


![figure](mode_selection.png)

* La sélection par le nom : Quand l'utilisateur choisit le mode séléction par nom, l'application lui propose d'entrer un nom pour séléctioner  l'ensemble des photos qui ont le même nom ou qui contient une partie du nom entrer.

![figure](mode_selection_par_nom.png)

* La sélection par  date : Quand l'utilisateur choisit le mode séléction par date, l'application lui propose d'entrer un date selon une format précise pour séléctioner les photos qui ont la même date.

![figure](mode_selection_2.png)
![figure](mode_selection_par_date.png)
* La sélection par  localisation  :Quand l'utilisateur choisit le mode séléction par localisation, l'application lui propose d'entrer les paramètres suivant:longitude, latitude et altitudee pour séléctioner les photos qui ont la même localisation.

![figure](mode_selection_gps.png)
* La sélection par nom de l'appareil photo ou la camera  :Quand l'utilisateur choisit le mode séléction par appareil, l'application lui propose d'entrer son nom afin d'afficher les photos prisent par la même appareil.

![figure](mode_selection_appareil.png)

* La sélection par résolution:Quand l'utilisateur choisit le mode séléction par résolution, l'application lui propose d'entrer la résolution x et la résolution y pour afficher les photos qui ont la même résolution.

![figure](mode_selection_resolution.png)
 * La sélection par tags:Quand l'utilisateur choisit le mode séléction par tags, l'application lui propose d'entrer un tag pour séléctioner les photos qui ont le même tag.


![figure](validation_suppression.png)
* Ajouter tags : Durant chaque mode de séléction l'application propose à l'utilisateur d'ajouter un ou plusieurs tags.Dans le cas d'ajoute de plusieurs tags, il faudra séparer entre eux avec "/" lors de la saisie (même chose pour la suppresion).

![figure](ajout_comments.png)

![figure](ajout_tags.png)
* Supprimer tags : Durant chaque mode de séléction l'application propose à l'utilisateur de supprimer un ou plusieurs tags.

![figure](supprimer_tags.png)
