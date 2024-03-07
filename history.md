## v0.1.0
The initial version plays random moves.

Match results:
```
1+0.01s:
    Score of Rawr-0.1.0 vs Random: 149 - 133 - 1718  [0.504] 2000
    Elo difference: 2.8 +/- 5.7, LOS: 83.0 %, DrawRatio: 85.9 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins      23 (4.60%)
- draws    442 (88.40%)
- losses    35 (7.00%)
Styles:
Aggression value 0.023 weight 4.0 weighted 0.093 contribution 1.69% name Game length
Aggression value 0.198 weight 2.0 weighted 0.396 contribution 7.19% name Capture early
Aggression value 0.246 weight 4.0 weighted 0.985 contribution 17.87% name Capture near king
Aggression value 0.179 weight 2.0 weighted 0.358 contribution 6.50% name Move near king
Aggression value 0.070 weight 0.2 weighted 0.014 contribution 0.25% name Castle opposite
Aggression value 0.032 weight 1.0 weighted 0.032 contribution 0.58% name Push pawns
Aggression value 0.055 weight 5.0 weighted 0.276 contribution 5.01% name Checks
Aggression value 0.130 weight 5.0 weighted 0.652 contribution 11.83% name Win while behind
Aggression value 0.078 weight 5.0 weighted 0.391 contribution 7.09% name Capture frequency
Aggression value 0.350 weight 4.0 weighted 1.402 contribution 25.42% name Push pawns towards king
Aggression value 0.140 weight 4.0 weighted 0.560 contribution 10.17% name Rook/Queen threats on king
Aggression value 0.088 weight 4.0 weighted 0.353 contribution 6.40% name Bishop/Queen threats on king
- Aggressive   0.274
```

---

## v0.2.0
1 ply search with material eval.

Match results:
```
1+0.01s:
    Score of Rawr-0.2.0 vs Rawr-0.1.0: 635 - 2 - 1363  [0.658] 2000
    Elo difference: 113.9 +/- 7.9, LOS: 100.0 %, DrawRatio: 68.2 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins       4 (0.80%)
- draws    492 (98.40%)
- losses     4 (0.80%)
Styles:
Aggression value 0.805 weight 4.0 weighted 3.221 contribution 33.50% name Game length
Aggression value 0.635 weight 2.0 weighted 1.270 contribution 13.20% name Capture early
Aggression value 0.194 weight 4.0 weighted 0.776 contribution 8.06% name Capture near king
Aggression value 0.068 weight 2.0 weighted 0.137 contribution 1.42% name Move near king
Aggression value 0.000 weight 0.2 weighted 0.000 contribution 0.00% name Castle opposite
Aggression value 0.049 weight 1.0 weighted 0.049 contribution 0.51% name Push pawns
Aggression value 0.046 weight 5.0 weighted 0.230 contribution 2.39% name Checks
Aggression value 0.000 weight 5.0 weighted 0.000 contribution 0.00% name Win while behind
Aggression value 0.346 weight 5.0 weighted 1.729 contribution 17.98% name Capture frequency
Aggression value 0.409 weight 4.0 weighted 1.636 contribution 17.01% name Push pawns towards king
Aggression value 0.079 weight 4.0 weighted 0.315 contribution 3.28% name Rook/Queen threats on king
Aggression value 0.063 weight 4.0 weighted 0.254 contribution 2.64% name Bishop/Queen threats on king
- Aggressive   0.478
```

---

## v0.3.0
Add gameover score awareness.

Match results:
```
1+0.01s:
    Score of Rawr-0.3.0 vs Rawr-0.2.0: 426 - 19 - 1555  [0.602] 2000
    Elo difference: 71.7 +/- 6.8, LOS: 100.0 %, DrawRatio: 77.8 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins      91 (18.20%)
- draws    318 (63.60%)
- losses    91 (18.20%)
Styles:
Aggression value 0.651 weight 4.0 weighted 2.604 contribution 29.71% name Game length
Aggression value 0.623 weight 2.0 weighted 1.247 contribution 14.22% name Capture early
Aggression value 0.196 weight 4.0 weighted 0.786 contribution 8.96% name Capture near king
Aggression value 0.065 weight 2.0 weighted 0.130 contribution 1.48% name Move near king
Aggression value 0.052 weight 0.2 weighted 0.010 contribution 0.12% name Castle opposite
Aggression value 0.050 weight 1.0 weighted 0.050 contribution 0.57% name Push pawns
Aggression value 0.046 weight 5.0 weighted 0.229 contribution 2.61% name Checks
Aggression value 0.011 weight 5.0 weighted 0.055 contribution 0.63% name Win while behind
Aggression value 0.294 weight 5.0 weighted 1.472 contribution 16.79% name Capture frequency
Aggression value 0.412 weight 4.0 weighted 1.647 contribution 18.79% name Push pawns towards king
Aggression value 0.074 weight 4.0 weighted 0.297 contribution 3.39% name Rook/Queen threats on king
Aggression value 0.060 weight 4.0 weighted 0.239 contribution 2.72% name Bishop/Queen threats on king
- Aggressive   0.436
```

---

## v0.4.0
Depth 2 negamax.

Match results:
```
1+0.01s:
    Score of Rawr-0.4.0 vs Rawr-0.3.0: 1686 - 19 - 295  [0.917] 2000
    Elo difference: 416.7 +/- 19.9, LOS: 100.0 %, DrawRatio: 14.8 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     135 (27.00%)
- draws    230 (46.00%)
- losses   135 (27.00%)
Styles:
Aggression value 0.816 weight 4.0 weighted 3.263 contribution 32.28% name Game length
Aggression value 0.552 weight 2.0 weighted 1.104 contribution 10.93% name Capture early
Aggression value 0.232 weight 4.0 weighted 0.930 contribution 9.20% name Capture near king
Aggression value 0.073 weight 2.0 weighted 0.147 contribution 1.45% name Move near king
Aggression value 0.000 weight 0.2 weighted 0.000 contribution 0.00% name Castle opposite
Aggression value 0.057 weight 1.0 weighted 0.057 contribution 0.57% name Push pawns
Aggression value 0.114 weight 5.0 weighted 0.572 contribution 5.66% name Checks
Aggression value 0.148 weight 5.0 weighted 0.741 contribution 7.33% name Win while behind
Aggression value 0.204 weight 5.0 weighted 1.018 contribution 10.07% name Capture frequency
Aggression value 0.394 weight 4.0 weighted 1.576 contribution 15.59% name Push pawns towards king
Aggression value 0.079 weight 4.0 weighted 0.315 contribution 3.12% name Rook/Queen threats on king
Aggression value 0.096 weight 4.0 weighted 0.384 contribution 3.80% name Bishop/Queen threats on king
- Aggressive   0.503
```

---

## v0.5.0
Negamax.

Match results:
```
1+0.01s:
    Score of Rawr-0.5.0 vs Rawr-0.4.0: 190 - 2 - 8  [0.970] 200
    Elo difference: 603.9 +/- 134.4, LOS: 100.0 %, DrawRatio: 4.0 %
10+0.1s:
    Score of Rawr-0.5.0 vs Rawr-0.4.0: 46 - 1 - 3  [0.950] 50
    Elo difference: 511.5 +/- 607.4, LOS: 100.0 %, DrawRatio: 6.0 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     207 (41.40%)
- draws     84 (16.80%)
- losses   209 (41.80%)
Styles:
Aggression value 0.254 weight 4.0 weighted 1.015 contribution 16.18% name Game length
Aggression value 0.386 weight 2.0 weighted 0.773 contribution 12.32% name Capture early
Aggression value 0.221 weight 4.0 weighted 0.883 contribution 14.07% name Capture near king
Aggression value 0.082 weight 2.0 weighted 0.163 contribution 2.60% name Move near king
Aggression value 0.019 weight 0.2 weighted 0.004 contribution 0.06% name Castle opposite
Aggression value 0.049 weight 1.0 weighted 0.049 contribution 0.79% name Push pawns
Aggression value 0.076 weight 5.0 weighted 0.382 contribution 6.09% name Checks
Aggression value 0.010 weight 5.0 weighted 0.048 contribution 0.77% name Win while behind
Aggression value 0.141 weight 5.0 weighted 0.703 contribution 11.21% name Capture frequency
Aggression value 0.375 weight 4.0 weighted 1.500 contribution 23.91% name Push pawns towards king
Aggression value 0.113 weight 4.0 weighted 0.453 contribution 7.22% name Rook/Queen threats on king
Aggression value 0.075 weight 4.0 weighted 0.300 contribution 4.78% name Bishop/Queen threats on king
- Aggressive   0.312
```

---

## v0.6.0
Alpha beta pruning.

Match results:
```
1+0.01s:
    Score of Rawr-0.6.0 vs Rawr-0.5.0: 452 - 15 - 33  [0.937] 500
    Elo difference: 469.0 +/- 54.6, LOS: 100.0 %, DrawRatio: 6.6 %
10+0.1s:
    Score of Rawr-0.6.0 vs Rawr-0.5.0: 40 - 2 - 8  [0.880] 50
    Elo difference: 346.1 +/- 132.0, LOS: 100.0 %, DrawRatio: 16.0 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     177 (35.40%)
- draws    143 (28.60%)
- losses   180 (36.00%)
Styles:
Aggression value 0.194 weight 4.0 weighted 0.774 contribution 12.70% name Game length
Aggression value 0.357 weight 2.0 weighted 0.714 contribution 11.71% name Capture early
Aggression value 0.225 weight 4.0 weighted 0.898 contribution 14.74% name Capture near king
Aggression value 0.086 weight 2.0 weighted 0.173 contribution 2.83% name Move near king
Aggression value 0.011 weight 0.2 weighted 0.002 contribution 0.03% name Castle opposite
Aggression value 0.053 weight 1.0 weighted 0.053 contribution 0.87% name Push pawns
Aggression value 0.079 weight 5.0 weighted 0.395 contribution 6.48% name Checks
Aggression value 0.023 weight 5.0 weighted 0.113 contribution 1.85% name Win while behind
Aggression value 0.130 weight 5.0 weighted 0.651 contribution 10.69% name Capture frequency
Aggression value 0.377 weight 4.0 weighted 1.506 contribution 24.71% name Push pawns towards king
Aggression value 0.130 weight 4.0 weighted 0.522 contribution 8.56% name Rook/Queen threats on king
Aggression value 0.073 weight 4.0 weighted 0.294 contribution 4.82% name Bishop/Queen threats on king
- Aggressive   0.303
```

---

## v0.7.0
PST.

Match results:
```
1+0.01s:
    Score of Rawr-0.7.0 vs Rawr-0.6.0: 230 - 85 - 85  [0.681] 400
    Elo difference: 131.9 +/- 31.8, LOS: 100.0 %, DrawRatio: 21.3 %
10+0.1s:
    Score of Rawr-0.7.0 vs Rawr-0.6.0: 31 - 9 - 10  [0.720] 50
    Elo difference: 164.1 +/- 96.7, LOS: 100.0 %, DrawRatio: 20.0 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     163 (32.60%)
- draws    156 (31.20%)
- losses   181 (36.20%)
Styles:
Aggression value 0.384 weight 4.0 weighted 1.536 contribution 19.09% name Game length
Aggression value 0.429 weight 2.0 weighted 0.858 contribution 10.66% name Capture early
Aggression value 0.273 weight 4.0 weighted 1.090 contribution 13.55% name Capture near king
Aggression value 0.173 weight 2.0 weighted 0.346 contribution 4.30% name Move near king
Aggression value 0.088 weight 0.2 weighted 0.018 contribution 0.22% name Castle opposite
Aggression value 0.030 weight 1.0 weighted 0.030 contribution 0.37% name Push pawns
Aggression value 0.095 weight 5.0 weighted 0.476 contribution 5.92% name Checks
Aggression value 0.067 weight 5.0 weighted 0.337 contribution 4.19% name Win while behind
Aggression value 0.125 weight 5.0 weighted 0.626 contribution 7.78% name Capture frequency
Aggression value 0.412 weight 4.0 weighted 1.648 contribution 20.49% name Push pawns towards king
Aggression value 0.178 weight 4.0 weighted 0.712 contribution 8.85% name Rook/Queen threats on king
Aggression value 0.092 weight 4.0 weighted 0.369 contribution 4.58% name Bishop/Queen threats on king
- Aggressive   0.400
```

---

## v0.8.0
MVV-LVA move ordering.

Match results:
```
1+0.01s:
    Score of Rawr-0.8.0 vs Rawr-0.7.0: 658 - 137 - 205  [0.760] 1000
    Elo difference: 200.7 +/- 21.4, LOS: 100.0 %, DrawRatio: 20.5 %
10+0.1s:
    Score of Rawr-0.8.0 vs Rawr-0.7.0: 69 - 14 - 17  [0.775] 100
    Elo difference: 214.8 +/- 72.8, LOS: 100.0 %, DrawRatio: 17.0 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     177 (35.40%)
- draws    133 (26.60%)
- losses   190 (38.00%)
Styles:
Aggression value 0.205 weight 4.0 weighted 0.819 contribution 12.21% name Game length
Aggression value 0.423 weight 2.0 weighted 0.847 contribution 12.63% name Capture early
Aggression value 0.228 weight 4.0 weighted 0.911 contribution 13.60% name Capture near king
Aggression value 0.107 weight 2.0 weighted 0.214 contribution 3.19% name Move near king
Aggression value 0.182 weight 0.2 weighted 0.036 contribution 0.54% name Castle opposite
Aggression value 0.033 weight 1.0 weighted 0.033 contribution 0.49% name Push pawns
Aggression value 0.079 weight 5.0 weighted 0.393 contribution 5.86% name Checks
Aggression value 0.006 weight 5.0 weighted 0.028 contribution 0.42% name Win while behind
Aggression value 0.139 weight 5.0 weighted 0.695 contribution 10.37% name Capture frequency
Aggression value 0.415 weight 4.0 weighted 1.658 contribution 24.74% name Push pawns towards king
Aggression value 0.174 weight 4.0 weighted 0.698 contribution 10.41% name Rook/Queen threats on king
Aggression value 0.093 weight 4.0 weighted 0.370 contribution 5.52% name Bishop/Queen threats on king
- Aggressive   0.333
```

---

## v0.9.0
Adjust mate scores by depth from root.

Match results:
```
1+0.01s:
    Score of Rawr-0.9.0 vs Rawr-0.8.0: 368 - 334 - 298  [0.517] 1000
    Elo difference: 11.8 +/- 18.0, LOS: 90.0 %, DrawRatio: 29.8 %
10+0.1s:
    Score of Rawr-0.9.0 vs Rawr-0.8.0: 231 - 233 - 136  [0.498] 600
    Elo difference: -1.2 +/- 24.5, LOS: 46.3 %, DrawRatio: 22.7 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     183 (36.60%)
- draws    121 (24.20%)
- losses   196 (39.20%)
Styles:
Aggression value 0.304 weight 4.0 weighted 1.215 contribution 16.52% name Game length
Aggression value 0.423 weight 2.0 weighted 0.846 contribution 11.51% name Capture early
Aggression value 0.224 weight 4.0 weighted 0.895 contribution 12.18% name Capture near king
Aggression value 0.105 weight 2.0 weighted 0.210 contribution 2.86% name Move near king
Aggression value 0.119 weight 0.2 weighted 0.024 contribution 0.32% name Castle opposite
Aggression value 0.033 weight 1.0 weighted 0.033 contribution 0.45% name Push pawns
Aggression value 0.081 weight 5.0 weighted 0.404 contribution 5.50% name Checks
Aggression value 0.011 weight 5.0 weighted 0.055 contribution 0.74% name Win while behind
Aggression value 0.161 weight 5.0 weighted 0.804 contribution 10.94% name Capture frequency
Aggression value 0.442 weight 4.0 weighted 1.769 contribution 24.07% name Push pawns towards king
Aggression value 0.175 weight 4.0 weighted 0.698 contribution 9.50% name Rook/Queen threats on king
Aggression value 0.100 weight 4.0 weighted 0.399 contribution 5.42% name Bishop/Queen threats on king
- Aggressive   0.366
```

---

## v0.10.0
Add qsearch.

Match results:
```
1+0.01s:
    Score of Rawr-0.10.0 vs Rawr-0.9.0: 842 - 62 - 96  [0.890] 1000
    Elo difference: 363.2 +/- 30.1, LOS: 100.0 %, DrawRatio: 9.6 %
10+0.1s:
    Score of Rawr-0.10.0 vs Rawr-0.9.0: 147 - 16 - 37  [0.828] 200
    Elo difference: 272.4 +/- 53.3, LOS: 100.0 %, DrawRatio: 18.5 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     184 (36.80%)
- draws    134 (26.80%)
- losses   182 (36.40%)
Styles:
Aggression value 0.199 weight 4.0 weighted 0.796 contribution 11.93% name Game length
Aggression value 0.389 weight 2.0 weighted 0.778 contribution 11.66% name Capture early
Aggression value 0.214 weight 4.0 weighted 0.857 contribution 12.85% name Capture near king
Aggression value 0.101 weight 2.0 weighted 0.202 contribution 3.04% name Move near king
Aggression value 0.141 weight 0.2 weighted 0.028 contribution 0.42% name Castle opposite
Aggression value 0.036 weight 1.0 weighted 0.036 contribution 0.53% name Push pawns
Aggression value 0.078 weight 5.0 weighted 0.388 contribution 5.81% name Checks
Aggression value 0.027 weight 5.0 weighted 0.136 contribution 2.04% name Win while behind
Aggression value 0.139 weight 5.0 weighted 0.697 contribution 10.45% name Capture frequency
Aggression value 0.411 weight 4.0 weighted 1.644 contribution 24.64% name Push pawns towards king
Aggression value 0.163 weight 4.0 weighted 0.651 contribution 9.76% name Rook/Queen threats on king
Aggression value 0.114 weight 4.0 weighted 0.457 contribution 6.85% name Bishop/Queen threats on king
- Aggressive   0.332
```

---

## v0.11.0
Check extensions.

Match results:
```
1+0.01s:
    Score of Rawr-0.11.0 vs Rawr-0.10.0: 458 - 233 - 309  [0.613] 1000
    Elo difference: 79.5 +/- 18.1, LOS: 100.0 %, DrawRatio: 30.9 %
10+0.1s:
    Score of Rawr-0.11.0 vs Rawr-0.10.0: 87 - 48 - 65  [0.598] 200
    Elo difference: 68.6 +/- 40.1, LOS: 100.0 %, DrawRatio: 32.5 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     164 (32.80%)
- draws    168 (33.60%)
- losses   168 (33.60%)
Styles:
Aggression value 0.212 weight 4.0 weighted 0.847 contribution 12.78% name Game length
Aggression value 0.380 weight 2.0 weighted 0.760 contribution 11.46% name Capture early
Aggression value 0.212 weight 4.0 weighted 0.848 contribution 12.79% name Capture near king
Aggression value 0.096 weight 2.0 weighted 0.191 contribution 2.88% name Move near king
Aggression value 0.093 weight 0.2 weighted 0.019 contribution 0.28% name Castle opposite
Aggression value 0.035 weight 1.0 weighted 0.035 contribution 0.53% name Push pawns
Aggression value 0.082 weight 5.0 weighted 0.409 contribution 6.17% name Checks
Aggression value 0.018 weight 5.0 weighted 0.091 contribution 1.38% name Win while behind
Aggression value 0.139 weight 5.0 weighted 0.693 contribution 10.44% name Capture frequency
Aggression value 0.415 weight 4.0 weighted 1.662 contribution 25.06% name Push pawns towards king
Aggression value 0.160 weight 4.0 weighted 0.642 contribution 9.67% name Rook/Queen threats on king
Aggression value 0.109 weight 4.0 weighted 0.435 contribution 6.56% name Bishop/Queen threats on king
- Aggressive   0.330
```

---

## v0.12.0
Passed pawns.

Match results:
```
1+0.01s:
    Score of Rawr-0.12.0 vs Rawr-0.11.0: 430 - 257 - 313  [0.587] 1000
    Elo difference: 60.7 +/- 18.0, LOS: 100.0 %, DrawRatio: 31.3 %
10+0.1s:
    Score of Rawr-0.12.0 vs Rawr-0.11.0: 93 - 48 - 59  [0.613] 200
    Elo difference: 79.5 +/- 41.2, LOS: 100.0 %, DrawRatio: 29.5 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     168 (33.60%)
- draws    166 (33.20%)
- losses   166 (33.20%)
Styles:
Aggression value 0.223 weight 4.0 weighted 0.891 contribution 13.47% name Game length
Aggression value 0.384 weight 2.0 weighted 0.768 contribution 11.61% name Capture early
Aggression value 0.218 weight 4.0 weighted 0.872 contribution 13.19% name Capture near king
Aggression value 0.107 weight 2.0 weighted 0.214 contribution 3.23% name Move near king
Aggression value 0.111 weight 0.2 weighted 0.022 contribution 0.34% name Castle opposite
Aggression value 0.035 weight 1.0 weighted 0.035 contribution 0.53% name Push pawns
Aggression value 0.082 weight 5.0 weighted 0.410 contribution 6.20% name Checks
Aggression value 0.000 weight 5.0 weighted 0.000 contribution 0.00% name Win while behind
Aggression value 0.150 weight 5.0 weighted 0.751 contribution 11.35% name Capture frequency
Aggression value 0.407 weight 4.0 weighted 1.630 contribution 24.64% name Push pawns towards king
Aggression value 0.162 weight 4.0 weighted 0.649 contribution 9.81% name Rook/Queen threats on king
Aggression value 0.093 weight 4.0 weighted 0.373 contribution 5.63% name Bishop/Queen threats on king
- Aggressive   0.329
```

---

## v0.13.0
Improve threefold detection.

Match results:
```
1+0.01s:
    Score of Rawr-0.13.0 vs Rawr-0.12.0: 666 - 593 - 741  [0.518] 2000
    Elo difference: 12.7 +/- 12.1, LOS: 98.0 %, DrawRatio: 37.0 %
10+0.1s:
    Score of Rawr-0.13.0 vs Rawr-0.12.0: 125 - 112 - 163  [0.516] 400
    Elo difference: 11.3 +/- 26.2, LOS: 80.1 %, DrawRatio: 40.8 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     156 (31.20%)
- draws    190 (38.00%)
- losses   154 (30.80%)
Styles:
Aggression value 0.252 weight 4.0 weighted 1.009 contribution 14.61% name Game length
Aggression value 0.386 weight 2.0 weighted 0.771 contribution 11.17% name Capture early
Aggression value 0.216 weight 4.0 weighted 0.864 contribution 12.51% name Capture near king
Aggression value 0.107 weight 2.0 weighted 0.214 contribution 3.09% name Move near king
Aggression value 0.086 weight 0.2 weighted 0.017 contribution 0.25% name Castle opposite
Aggression value 0.035 weight 1.0 weighted 0.035 contribution 0.51% name Push pawns
Aggression value 0.082 weight 5.0 weighted 0.411 contribution 5.96% name Checks
Aggression value 0.013 weight 5.0 weighted 0.064 contribution 0.93% name Win while behind
Aggression value 0.155 weight 5.0 weighted 0.777 contribution 11.26% name Capture frequency
Aggression value 0.427 weight 4.0 weighted 1.707 contribution 24.72% name Push pawns towards king
Aggression value 0.157 weight 4.0 weighted 0.629 contribution 9.11% name Rook/Queen threats on king
Aggression value 0.101 weight 4.0 weighted 0.405 contribution 5.87% name Bishop/Queen threats on king
- Aggressive   0.343
```

---

## v0.14.0
Rooks on open files.

Match results:
```
1+0.01s:
    Score of Rawr-0.14.0 vs Rawr-0.13.0: 717 - 625 - 658  [0.523] 2000
    Elo difference: 16.0 +/- 12.5, LOS: 99.4 %, DrawRatio: 32.9 %
10+0.1s:
    Score of Rawr-0.14.0 vs Rawr-0.13.0: 197 - 155 - 248  [0.535] 600
    Elo difference: 24.4 +/- 21.3, LOS: 98.7 %, DrawRatio: 41.3 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     157 (31.40%)
- draws    166 (33.20%)
- losses   177 (35.40%)
Styles:
Aggression value 0.259 weight 4.0 weighted 1.034 contribution 14.81% name Game length
Aggression value 0.404 weight 2.0 weighted 0.809 contribution 11.59% name Capture early
Aggression value 0.222 weight 4.0 weighted 0.890 contribution 12.75% name Capture near king
Aggression value 0.105 weight 2.0 weighted 0.209 contribution 3.00% name Move near king
Aggression value 0.177 weight 0.2 weighted 0.035 contribution 0.51% name Castle opposite
Aggression value 0.034 weight 1.0 weighted 0.034 contribution 0.49% name Push pawns
Aggression value 0.084 weight 5.0 weighted 0.418 contribution 5.99% name Checks
Aggression value 0.013 weight 5.0 weighted 0.064 contribution 0.91% name Win while behind
Aggression value 0.162 weight 5.0 weighted 0.811 contribution 11.62% name Capture frequency
Aggression value 0.414 weight 4.0 weighted 1.656 contribution 23.72% name Push pawns towards king
Aggression value 0.164 weight 4.0 weighted 0.655 contribution 9.39% name Rook/Queen threats on king
Aggression value 0.091 weight 4.0 weighted 0.364 contribution 5.22% name Bishop/Queen threats on king
- Aggressive   0.347
```

---

## v0.15.0
Phased eval.

Match results:
```
1+0.01s:
    Score of Rawr-0.15.0 vs Rawr-0.14.0: 583 - 453 - 164  [0.554] 1200
    Elo difference: 37.8 +/- 18.3, LOS: 100.0 %, DrawRatio: 13.7 %
10+0.1s:
    Score of Rawr-0.15.0 vs Rawr-0.14.0: 258 - 160 - 82  [0.598] 500
    Elo difference: 69.0 +/- 28.3, LOS: 100.0 %, DrawRatio: 16.4 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     207 (41.40%)
- draws     97 (19.40%)
- losses   196 (39.20%)
Styles:
Aggression value 0.331 weight 4.0 weighted 1.323 contribution 17.29% name Game length
Aggression value 0.403 weight 2.0 weighted 0.805 contribution 10.52% name Capture early
Aggression value 0.244 weight 4.0 weighted 0.976 contribution 12.75% name Capture near king
Aggression value 0.164 weight 2.0 weighted 0.327 contribution 4.27% name Move near king
Aggression value 0.070 weight 0.2 weighted 0.014 contribution 0.18% name Castle opposite
Aggression value 0.039 weight 1.0 weighted 0.039 contribution 0.50% name Push pawns
Aggression value 0.101 weight 5.0 weighted 0.504 contribution 6.59% name Checks
Aggression value 0.005 weight 5.0 weighted 0.024 contribution 0.32% name Win while behind
Aggression value 0.192 weight 5.0 weighted 0.958 contribution 12.52% name Capture frequency
Aggression value 0.398 weight 4.0 weighted 1.591 contribution 20.79% name Push pawns towards king
Aggression value 0.169 weight 4.0 weighted 0.677 contribution 8.85% name Rook/Queen threats on king
Aggression value 0.103 weight 4.0 weighted 0.414 contribution 5.41% name Bishop/Queen threats on king
- Aggressive   0.381
```

---

## v0.16.0
King pawn shield.

Match results:
```
1+0.01s:
    Score of Rawr-0.16.0 vs Rawr-0.15.0: 878 - 726 - 396  [0.538] 2000
    Elo difference: 26.5 +/- 13.7, LOS: 100.0 %, DrawRatio: 19.8 %
10+0.1s:
    Score of Rawr-0.16.0 vs Rawr-0.15.0: 426 - 334 - 240  [0.546] 1000
    Elo difference: 32.1 +/- 18.8, LOS: 100.0 %, DrawRatio: 24.0 %
```
Style script:
```
Filter: All games
Score:
- games    500
- wins     188 (37.60%)
- draws    121 (24.20%)
- losses   191 (38.20%)
Styles:
Aggression value 0.310 weight 4.0 weighted 1.239 contribution 16.43% name Game length
Aggression value 0.395 weight 2.0 weighted 0.791 contribution 10.49% name Capture early
Aggression value 0.240 weight 4.0 weighted 0.958 contribution 12.71% name Capture near king
Aggression value 0.163 weight 2.0 weighted 0.326 contribution 4.33% name Move near king
Aggression value 0.031 weight 0.2 weighted 0.006 contribution 0.08% name Castle opposite
Aggression value 0.036 weight 1.0 weighted 0.036 contribution 0.48% name Push pawns
Aggression value 0.098 weight 5.0 weighted 0.490 contribution 6.51% name Checks
Aggression value 0.027 weight 5.0 weighted 0.133 contribution 1.76% name Win while behind
Aggression value 0.191 weight 5.0 weighted 0.956 contribution 12.69% name Capture frequency
Aggression value 0.382 weight 4.0 weighted 1.528 contribution 20.27% name Push pawns towards king
Aggression value 0.170 weight 4.0 weighted 0.682 contribution 9.04% name Rook/Queen threats on king
Aggression value 0.098 weight 4.0 weighted 0.392 contribution 5.20% name Bishop/Queen threats on king
- Aggressive   0.375
```
