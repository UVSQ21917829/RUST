# Conception technique
## Dépendences 
le projet utilise la bibliothèque externe rexiv2 (version 0.9.0). Cette dépendance est disponible sur crates.io.

```toml
[dependencies]
rexiv2="0.9.0"
```
Étant donné que la bibliothèque rexiv2 est lié à gexiv2 et transitivement à Exiv2, rexiv2 dépend évidemment d'eux (et de leurs dépendances). L'installation de ces bibliothèques sur le système est une condition préalable à l'utilisation de rexiv2 ou de tout logiciel construit sur celle-ci.

# Sénario
