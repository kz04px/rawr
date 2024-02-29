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
