# Rust Learning Journey

Ce dépôt documente mon parcours d'apprentissage autodidacte du langage **Rust**.
Il regroupe différents exercices, algorithmes et structures de données implémentés avec un objectif clair : **apprendre à coder proprement (Clean Code) dès le début**.

## Philosophie du Projet

Mon approche ne consiste pas seulement à faire fonctionner le code, mais à comprendre le "pourquoi" et le "comment" de Rust :

- **Rigueur :** Respect des conventions de nommage et du formatage Rust.
- **Compréhension profonde :** Maîtrise de la gestion mémoire (Ownership/Borrowing) plutôt que de la contourner.
- **Architecture :** Séparation des responsabilités et encapsulation logique.

## Concepts Maîtrisés

À travers divers exercices (convertisseurs, gestionnaires de listes, modélisation de données), j'ai mis en pratique les notions suivantes :

### 1. Les Fondamentaux & Types

- Typage fort (`u8`, `i32`, `f64`) et conversion explicite.
- Le concept de **Shadowing** pour transformer proprement les données.
- Gestion des entrées utilisateurs (`std::io`) et parsing sécurisé.

### 2. Contrôle du Flux

- Utilisation idiomatique de **`match`** pour gérer les cas multiples.
- Boucles (`loop`, `for`) et itération sur des collections.
- Gestion des erreurs de base ( `Result`, `.expect()`).

### 3. Gestion de la Mémoire

- **Ownership :** Comprendre quand une variable est déplacée ou copiée.
- **Borrowing :**
  - Références partagées (`&T`) pour la lecture.
  - Références mutables (`&mut T`) pour la modification.
- Règles strictes de l'emprunt.

### 4. Structures de Données & POO (Style Rust)

- Création de **`struct`** personnalisées.
- Blocs **`impl`** pour l'encapsulation.
- Différence entre **Méthodes** (prenant `&self`) et **Fonctions Associées** (constructeurs `new`).
- Composition d'objets (Struct dans Struct).

## Exemples d'Exercices Réalisés

- **Calculateurs :** Conversion de températures avec gestion précise des types flottants.
- **Collections :** Gestion dynamique de listes (Vecteurs) avec allocation mémoire.
- **Systèmes Logiques :** Modélisation d'entités (ex: Utilisateurs, Comptes) avec interaction et mutation d'état.

## La suite à venir

## Utilisation

Pour tester le code actuel :

```bash
cargo run
```
