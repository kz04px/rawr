# Rawr
A chess engine designed to play in an aggressive style using statistics.

---

## Style script
The style script is written in Python and found [here](tools/style/style.py). Use `--help` for an overview of available commands.

Example usage:
```
python ./style.py --pgn=./kasparov.pgn --player="Kasparov, Garry" --all
```
Example output:
```
Filter: All games
Score:
- games   2000
- wins    1000 (50.00%)
- draws    850 (42.50%)
- losses   150 (7.50%)
Styles:
- Aggressive   0.500
- Positional   0.400
```

---

## Thanks:
- For ideas and general conversation about the project: [Lana](https://github.com/princesslana/princhess), [Serdra](https://github.com/Serdra)
