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
Filter: White
Score:
- games   1000
- wins      73 (7.30%)
- draws    851 (85.10%)
- losses    76 (7.60%)
Styles:
Aggression value 0.040 weight 4.0 weighted 0.161 contribution 4.93% name Game length
Aggression value 0.187 weight 2.0 weighted 0.375 contribution 11.45% name Capture early
Aggression value 0.247 weight 4.0 weighted 0.988 contribution 30.19% name Capture near king
Aggression value 0.177 weight 2.0 weighted 0.354 contribution 10.82% name Move near king
Aggression value 0.073 weight 0.2 weighted 0.015 contribution 0.44% name Castle opposite
Aggression value 0.032 weight 1.0 weighted 0.032 contribution 0.98% name Push pawns
Aggression value 0.054 weight 5.0 weighted 0.272 contribution 8.32% name Checks
Aggression value 0.137 weight 5.0 weighted 0.685 contribution 20.94% name Win while behind
Aggression value 0.078 weight 5.0 weighted 0.390 contribution 11.92% name Capture frequency
- Aggressive   0.232
```
```
Filter: Black
Score:
- games   1000
- wins      86 (8.60%)
- draws    826 (82.60%)
- losses    88 (8.80%)
Styles:
Aggression value 0.042 weight 4.0 weighted 0.167 contribution 5.54% name Game length
Aggression value 0.203 weight 2.0 weighted 0.406 contribution 13.49% name Capture early
Aggression value 0.247 weight 4.0 weighted 0.989 contribution 32.84% name Capture near king
Aggression value 0.180 weight 2.0 weighted 0.360 contribution 11.95% name Move near king
Aggression value 0.056 weight 0.2 weighted 0.011 contribution 0.37% name Castle opposite
Aggression value 0.033 weight 1.0 weighted 0.033 contribution 1.09% name Push pawns
Aggression value 0.058 weight 5.0 weighted 0.289 contribution 9.59% name Checks
Aggression value 0.070 weight 5.0 weighted 0.349 contribution 11.59% name Win while behind
Aggression value 0.082 weight 5.0 weighted 0.408 contribution 13.55% name Capture frequency
- Aggressive   0.214
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
Filter: White
Score:
- games   1000
- wins       8 (0.80%)
- draws    980 (98.00%)
- losses    12 (1.20%)
Styles:
Aggression value 0.808 weight 4.0 weighted 3.230 contribution 43.47% name Game length
Aggression value 0.626 weight 2.0 weighted 1.251 contribution 16.84% name Capture early
Aggression value 0.206 weight 4.0 weighted 0.824 contribution 11.09% name Capture near king
Aggression value 0.070 weight 2.0 weighted 0.140 contribution 1.88% name Move near king
Aggression value 0.005 weight 0.2 weighted 0.001 contribution 0.01% name Castle opposite
Aggression value 0.049 weight 1.0 weighted 0.049 contribution 0.66% name Push pawns
Aggression value 0.047 weight 5.0 weighted 0.233 contribution 3.14% name Checks
Aggression value 0.000 weight 5.0 weighted 0.000 contribution 0.00% name Win while behind
Aggression value 0.340 weight 5.0 weighted 1.702 contribution 22.90% name Capture frequency
- Aggressive   0.527
```
```
Filter: Black
Score:
- games   1000
- wins      12 (1.20%)
- draws    980 (98.00%)
- losses     8 (0.80%)
Styles:
Aggression value 0.808 weight 4.0 weighted 3.230 contribution 43.43% name Game length
Aggression value 0.635 weight 2.0 weighted 1.270 contribution 17.08% name Capture early
Aggression value 0.189 weight 4.0 weighted 0.758 contribution 10.19% name Capture near king
Aggression value 0.069 weight 2.0 weighted 0.138 contribution 1.85% name Move near king
Aggression value 0.005 weight 0.2 weighted 0.001 contribution 0.01% name Castle opposite
Aggression value 0.048 weight 1.0 weighted 0.048 contribution 0.64% name Push pawns
Aggression value 0.046 weight 5.0 weighted 0.232 contribution 3.11% name Checks
Aggression value 0.000 weight 5.0 weighted 0.000 contribution 0.00% name Win while behind
Aggression value 0.352 weight 5.0 weighted 1.761 contribution 23.67% name Capture frequency
- Aggressive   0.528
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
Filter: White
Score:
- games   1000
- wins     118 (11.80%)
- draws    692 (69.20%)
- losses   190 (19.00%)
Styles:
Aggression value 0.657 weight 4.0 weighted 2.626 contribution 40.46% name Game length
Aggression value 0.623 weight 2.0 weighted 1.245 contribution 19.18% name Capture early
Aggression value 0.205 weight 4.0 weighted 0.819 contribution 12.62% name Capture near king
Aggression value 0.061 weight 2.0 weighted 0.122 contribution 1.88% name Move near king
Aggression value 0.029 weight 0.2 weighted 0.006 contribution 0.09% name Castle opposite
Aggression value 0.050 weight 1.0 weighted 0.050 contribution 0.78% name Push pawns
Aggression value 0.041 weight 5.0 weighted 0.207 contribution 3.19% name Checks
Aggression value 0.000 weight 5.0 weighted 0.000 contribution 0.00% name Win while behind
Aggression value 0.283 weight 5.0 weighted 1.415 contribution 21.80% name Capture frequency
- Aggressive   0.460
```
```
Filter: Black
Score:
- games   1000
- wins     190 (19.00%)
- draws    692 (69.20%)
- losses   118 (11.80%)
Styles:
Aggression value 0.657 weight 4.0 weighted 2.626 contribution 39.63% name Game length
Aggression value 0.634 weight 2.0 weighted 1.268 contribution 19.13% name Capture early
Aggression value 0.197 weight 4.0 weighted 0.788 contribution 11.89% name Capture near king
Aggression value 0.065 weight 2.0 weighted 0.130 contribution 1.96% name Move near king
Aggression value 0.029 weight 0.2 weighted 0.006 contribution 0.09% name Castle opposite
Aggression value 0.048 weight 1.0 weighted 0.048 contribution 0.73% name Push pawns
Aggression value 0.049 weight 5.0 weighted 0.247 contribution 3.73% name Checks
Aggression value 0.005 weight 5.0 weighted 0.026 contribution 0.40% name Win while behind
Aggression value 0.298 weight 5.0 weighted 1.488 contribution 22.45% name Capture frequency
- Aggressive   0.470
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
Filter: White
Score:
- games   1000
- wins     314 (31.40%)
- draws    386 (38.60%)
- losses   300 (30.00%)
Styles:
Aggression value 0.784 weight 4.0 weighted 3.136 contribution 40.85% name Game length
Aggression value 0.529 weight 2.0 weighted 1.058 contribution 13.78% name Capture early
Aggression value 0.238 weight 4.0 weighted 0.951 contribution 12.39% name Capture near king
Aggression value 0.076 weight 2.0 weighted 0.152 contribution 1.99% name Move near king
Aggression value 0.016 weight 0.2 weighted 0.003 contribution 0.04% name Castle opposite
Aggression value 0.058 weight 1.0 weighted 0.058 contribution 0.76% name Push pawns
Aggression value 0.118 weight 5.0 weighted 0.588 contribution 7.66% name Checks
Aggression value 0.140 weight 5.0 weighted 0.701 contribution 9.13% name Win while behind
Aggression value 0.206 weight 5.0 weighted 1.030 contribution 13.41% name Capture frequency
- Aggressive   0.544
```
```
Filter: Black
Score:
- games   1000
- wins     300 (30.00%)
- draws    386 (38.60%)
- losses   314 (31.40%)
Styles:
Aggression value 0.784 weight 4.0 weighted 3.136 contribution 41.60% name Game length
Aggression value 0.549 weight 2.0 weighted 1.099 contribution 14.57% name Capture early
Aggression value 0.220 weight 4.0 weighted 0.879 contribution 11.66% name Capture near king
Aggression value 0.075 weight 2.0 weighted 0.150 contribution 1.99% name Move near king
Aggression value 0.016 weight 0.2 weighted 0.003 contribution 0.04% name Castle opposite
Aggression value 0.057 weight 1.0 weighted 0.057 contribution 0.75% name Push pawns
Aggression value 0.118 weight 5.0 weighted 0.588 contribution 7.81% name Checks
Aggression value 0.117 weight 5.0 weighted 0.583 contribution 7.74% name Win while behind
Aggression value 0.209 weight 5.0 weighted 1.043 contribution 13.84% name Capture frequency
- Aggressive   0.535
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
Filter: White
Score:
- games    200
- wins      62 (31.00%)
- draws     41 (20.50%)
- losses    97 (48.50%)
Styles:
Aggression value 0.300 weight 4.0 weighted 1.198 contribution 27.74% name Game length
Aggression value 0.401 weight 2.0 weighted 0.801 contribution 18.55% name Capture early
Aggression value 0.220 weight 4.0 weighted 0.880 contribution 20.36% name Capture near king
Aggression value 0.079 weight 2.0 weighted 0.157 contribution 3.64% name Move near king
Aggression value 0.125 weight 0.2 weighted 0.025 contribution 0.58% name Castle opposite
Aggression value 0.047 weight 1.0 weighted 0.047 contribution 1.09% name Push pawns
Aggression value 0.068 weight 5.0 weighted 0.342 contribution 7.92% name Checks
Aggression value 0.032 weight 5.0 weighted 0.161 contribution 3.73% name Win while behind
Aggression value 0.142 weight 5.0 weighted 0.708 contribution 16.38% name Capture frequency
- Aggressive   0.306
```
```
Filter: Black
Score:
- games    200
- wins     104 (52.00%)
- draws     41 (20.50%)
- losses    55 (27.50%)
Styles:
Aggression value 0.295 weight 4.0 weighted 1.182 contribution 27.35% name Game length
Aggression value 0.409 weight 2.0 weighted 0.818 contribution 18.94% name Capture early
Aggression value 0.230 weight 4.0 weighted 0.918 contribution 21.26% name Capture near king
Aggression value 0.093 weight 2.0 weighted 0.186 contribution 4.31% name Move near king
Aggression value 0.100 weight 0.2 weighted 0.020 contribution 0.46% name Castle opposite
Aggression value 0.050 weight 1.0 weighted 0.050 contribution 1.15% name Push pawns
Aggression value 0.085 weight 5.0 weighted 0.425 contribution 9.83% name Checks
Aggression value 0.000 weight 5.0 weighted 0.000 contribution 0.00% name Win while behind
Aggression value 0.144 weight 5.0 weighted 0.721 contribution 16.68% name Capture frequency
- Aggressive   0.306
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
Filter: White
Score:
- games    250
- wins      80 (32.00%)
- draws     79 (31.60%)
- losses    91 (36.40%)
Styles:
Aggression value 0.185 weight 4.0 weighted 0.739 contribution 20.12% name Game length
Aggression value 0.349 weight 2.0 weighted 0.697 contribution 18.99% name Capture early
Aggression value 0.230 weight 4.0 weighted 0.921 contribution 25.10% name Capture near king
Aggression value 0.086 weight 2.0 weighted 0.171 contribution 4.66% name Move near king
Aggression value 0.000 weight 0.2 weighted 0.000 contribution 0.00% name Castle opposite
Aggression value 0.054 weight 1.0 weighted 0.054 contribution 1.47% name Push pawns
Aggression value 0.078 weight 5.0 weighted 0.390 contribution 10.63% name Checks
Aggression value 0.013 weight 5.0 weighted 0.062 contribution 1.70% name Win while behind
Aggression value 0.127 weight 5.0 weighted 0.636 contribution 17.33% name Capture frequency
- Aggressive   0.260
```
```
Filter: Black
Score:
- games    250
- wins      97 (38.80%)
- draws     64 (25.60%)
- losses    89 (35.60%)
Styles:
Aggression value 0.202 weight 4.0 weighted 0.809 contribution 20.93% name Game length
Aggression value 0.365 weight 2.0 weighted 0.731 contribution 18.89% name Capture early
Aggression value 0.219 weight 4.0 weighted 0.875 contribution 22.63% name Capture near king
Aggression value 0.087 weight 2.0 weighted 0.174 contribution 4.50% name Move near king
Aggression value 0.021 weight 0.2 weighted 0.004 contribution 0.11% name Castle opposite
Aggression value 0.052 weight 1.0 weighted 0.052 contribution 1.34% name Push pawns
Aggression value 0.080 weight 5.0 weighted 0.400 contribution 10.34% name Checks
Aggression value 0.031 weight 5.0 weighted 0.155 contribution 4.00% name Win while behind
Aggression value 0.133 weight 5.0 weighted 0.667 contribution 17.26% name Capture frequency
- Aggressive   0.274
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
Filter: White
Score:
- games    250
- wins      76 (30.40%)
- draws     67 (26.80%)
- losses   107 (42.80%)
Styles:
Aggression value 0.406 weight 4.0 weighted 1.625 contribution 29.87% name Game length
Aggression value 0.423 weight 2.0 weighted 0.846 contribution 15.55% name Capture early
Aggression value 0.275 weight 4.0 weighted 1.099 contribution 20.20% name Capture near king
Aggression value 0.176 weight 2.0 weighted 0.353 contribution 6.48% name Move near king
Aggression value 0.088 weight 0.2 weighted 0.018 contribution 0.32% name Castle opposite
Aggression value 0.030 weight 1.0 weighted 0.030 contribution 0.56% name Push pawns
Aggression value 0.090 weight 5.0 weighted 0.450 contribution 8.26% name Checks
Aggression value 0.079 weight 5.0 weighted 0.395 contribution 7.25% name Win while behind
Aggression value 0.125 weight 5.0 weighted 0.626 contribution 11.51% name Capture frequency
```
```
Filter: Black
Score:
- games    250
- wins      87 (34.80%)
- draws     89 (35.60%)
- losses    74 (29.60%)
Styles:
Aggression value 0.362 weight 4.0 weighted 1.447 contribution 27.82% name Game length
Aggression value 0.434 weight 2.0 weighted 0.869 contribution 16.71% name Capture early
Aggression value 0.270 weight 4.0 weighted 1.081 contribution 20.79% name Capture near king
Aggression value 0.170 weight 2.0 weighted 0.340 contribution 6.54% name Move near king
Aggression value 0.088 weight 0.2 weighted 0.018 contribution 0.34% name Castle opposite
Aggression value 0.030 weight 1.0 weighted 0.030 contribution 0.58% name Push pawns
Aggression value 0.100 weight 5.0 weighted 0.502 contribution 9.66% name Checks
Aggression value 0.057 weight 5.0 weighted 0.287 contribution 5.53% name Win while behind
Aggression value 0.125 weight 5.0 weighted 0.626 contribution 12.03% name Capture frequency
```

---

## v0.8.0
MVV-LVA move ordering.

Match results:
```
1+0.01s:
    Score of Rawr-0.8.0 vs Rawr-0.7.0: 330 - 84 - 86  [0.746] 500
    Elo difference: 187.2 +/- 30.9, LOS: 100.0 %, DrawRatio: 17.2 %
10+0.1s:
    Score of Rawr-0.8.0 vs Rawr-0.7.0: 32 - 9 - 9  [0.730] 50
    Elo difference: 172.8 +/- 99.4, LOS: 100.0 %, DrawRatio: 18.0 %
```
Style script:
```
Filter: White
Score:
- games    250
- wins      99 (39.60%)
- draws     59 (23.60%)
- losses    92 (36.80%)
Styles:
Aggression value 0.339 weight 4.0 weighted 1.357 contribution 28.39% name Game length
Aggression value 0.425 weight 2.0 weighted 0.850 contribution 17.79% name Capture early
Aggression value 0.228 weight 4.0 weighted 0.910 contribution 19.04% name Capture near king
Aggression value 0.116 weight 2.0 weighted 0.232 contribution 4.85% name Move near king
Aggression value 0.107 weight 0.2 weighted 0.021 contribution 0.45% name Castle opposite
Aggression value 0.034 weight 1.0 weighted 0.034 contribution 0.71% name Push pawns
Aggression value 0.098 weight 5.0 weighted 0.488 contribution 10.21% name Checks
Aggression value 0.030 weight 5.0 weighted 0.152 contribution 3.17% name Win while behind
Aggression value 0.147 weight 5.0 weighted 0.736 contribution 15.40% name Capture frequency
```
```
Filter: Black
Score:
- games    250
- wins      79 (31.60%)
- draws     82 (32.80%)
- losses    89 (35.60%)
Styles:
Aggression value 0.320 weight 4.0 weighted 1.280 contribution 29.06% name Game length
Aggression value 0.435 weight 2.0 weighted 0.871 contribution 19.76% name Capture early
Aggression value 0.212 weight 4.0 weighted 0.846 contribution 19.21% name Capture near king
Aggression value 0.105 weight 2.0 weighted 0.211 contribution 4.79% name Move near king
Aggression value 0.131 weight 0.2 weighted 0.026 contribution 0.59% name Castle opposite
Aggression value 0.034 weight 1.0 weighted 0.034 contribution 0.77% name Push pawns
Aggression value 0.080 weight 5.0 weighted 0.401 contribution 9.11% name Checks
Aggression value 0.013 weight 5.0 weighted 0.063 contribution 1.44% name Win while behind
Aggression value 0.135 weight 5.0 weighted 0.673 contribution 15.27% name Capture frequency
```
