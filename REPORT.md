# Killer Sudoku Solver verslag

Naam: Michael Neys
Studentennummer: 2467626


## Implementatie & Gemaakte Keuzes
Om de code een beetje overzichtelijk te houden en makkelijk te kunnen debuggen, heb ik het project opgesplitst in meerdere bestanden `main.rs`, `models.rs`, `grid.rs` en `solver.rs`.

### JSON
In plaats van constant met de geneste JSON-structuren te werken tijdens het oplossen, vertaal ik eerst de invoer naar een simpele 2D-array van bytes (`[[u8; 9]; 9]`). Cages worden bewaard in een `SudokuMeta` struct. Om te voorkomen dat de solver telkens moet zoeken in welke cage een cel zit, gebruik ik een lookup-tabel (`cell_to_cage`).

Bij het schrijven naar JSON maak ik een clone van het originele invoerbestand en `clear` de given en loop dan over mijn solution om zo row, col en value terug in te vullen. Zo moet ik de cages niet overlopen om ook bij in de JSON te schrijven.

Voor JSON lees en schrijf opdrachten heb ik gebruik gemaakt van `serde`.

### optimalisaties
Om niet heel het bord te moeten checken heb ik de functie `is_valid_at` gemaakt die checkt of een bepaalde cell wel valid is door het probleem te bekijken in zijn rij, kolom, block en cage. Deze functie maakt gebruik van de functie `check_house` waarin het reken werk gebeurd (we stoppen als onderstaande waar zijn):
- checken of de som groter is dan de target
- als er geen lege cellen meer zijn en de target is niet gehaald
- als we het met de maximum waarde er niet komen en als we met de minimum waarde voor een cel er al over zitten

### recursie bij oplossen
Ik ben recursief tewerk gegaan in de functie `solve_recursive` door eerst de beste mogelijke cellen te zoeken en vanuit daar telkens weer verder te gaan met de cellen die de minste opties hebben. Als we bv 1 oplossing hebben voor een cell dan moeten we niet meer verder zoeken en vullen we die al in in de nieuwe grid. In deze functie wordt er ook rekening gehouden of we de `--seq` flag meegeven of als we al in een bepaalde diepte van de recursie zitten.

Voor parallelisme heb ik gebruik gemaakt van `rayon` met `into_par_iter`. Ik heb de grens gesteld op 4 lagen diep omdat ik merkte dat dat iets sneller was dan 3 of minder bij de moeilijke JSON die wij kregen.

## Tests
Ik heb in `grid.rs` en `solver.rs` testen geschreven voor de basis functionaliteit van mijn functies en/of helpfuncties te testen.

## Commentaar / verwerkte feedback
Uit feedback van het vorige Rust project heb ik proberen rekening te houden om bij elke functie toch comments te plaatsen dat we snel weten wat die functie doet.

# Tijd
Ondanks dat ik weinig tijd had door andere projectwerken en de verkiezingsweek van de studentenvereniging van Informatica is het mij toch gelukt om het project te maken, ik ben er zeker een goede 17 uur in totaal mee bezig geweest.

# Folder structuur
De implementatie moet in folder `/` staan, voor orde heb ik de bronbestanden in de subfolder `/src/` staan en is het project uitvoerbaar vanaf de locatie `/`. Hier was enige onduidelijkheid over in de klas.