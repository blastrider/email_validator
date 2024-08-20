# Email Validator

Ce projet est un outil en ligne de commande simple écrit en Rust qui vérifie si une adresse email est en minuscules et valide sa structure.

## Structure du Projet

```
email_validator/
├── Cargo.toml
└── src
    ├── main.rs
    └── email.rs
```

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) doit être installé sur votre machine.

## Installation

Clonez ce dépôt et accédez au répertoire du projet :

```bash
git clone https://github.com/votre-utilisateur/email_validator.git
cd email_validator
```

Ensuite, compilez le projet en utilisant Cargo :

```bash
cargo build --release
```

## Utilisation

Une fois le projet compilé, vous pouvez exécuter l'outil en ligne de commande comme suit :

```bash
./target/release/email_validator <email>
```

### Exemples

```bash
$ ./target/release/email_validator example@example.com
'example@example.com' est une adresse mail valide.

$ ./target/release/email_validator Example@Example.com
'Example@Example.com' n'est pas en minuscules.
```

## Fonctionnalités

- **Validation de la casse** : L'outil vérifie que l'adresse email est entièrement en minuscules. Si ce n'est pas le cas, il retourne un message d'erreur.
- **Validation de la structure** : Si l'adresse est en minuscules, l'outil vérifie ensuite si elle correspond à un format d'email valide à l'aide d'une expression régulière.

## Contribuer

Les contributions sont les bienvenues ! N'hésitez pas à soumettre des pull requests ou à ouvrir des issues pour discuter des améliorations potentielles.

## Licence

Ce projet est sous licence MIT. Consultez le fichier `LICENSE` pour plus d'informations.

## Documentation a completer
