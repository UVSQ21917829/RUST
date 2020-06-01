# Conception technique
## Dépendences 
le projet utilise la bibliothèque externe rexiv2 (version 0.9.0). Cette dépendance est disponible sur crates.io.

```toml
[dependencies]
rexiv2="0.9.0"
```
Cette bibliothèque fournit un wrapper Rust de la bibliothèque gexiv2, qui est elle-même un wrapper basé sur GObject de la bibliothèque Exiv2, qui fournit un accès en lecture et en écriture aux métadonnées Exif, XMP et IPTC pour les fichiers multimédias dans différents formats.

Étant donné que la bibliothèque rexiv2 est lié à gexiv2 et transitivement à Exiv2, rexiv2 dépend évidemment d'eux (et de leurs dépendances). L'installation de ces bibliothèques sur le système est une condition préalable à l'utilisation de rexiv2 ou de tout logiciel construit sur celle-ci.

# Sénario
