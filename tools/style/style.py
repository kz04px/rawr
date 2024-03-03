import time
import chess
import chess.pgn
import argparse
from dataclasses import dataclass, field


@dataclass
class Stats:
    num_wins: int = 0
    num_draws: int = 0
    num_losses: int = 0
    num_games: int = 0
    # Castling turn
    castle_first: int = 0
    castle_second: int = 0
    castle_never: int = 0
    # Castling side
    castle_king: int = 0
    castle_queen: int = 0
    # Castling thing
    castle_same: int = 0
    castle_opposite: int = 0
    # Move types
    total_captures: int = 0
    total_noncaptures: int = 0
    total_moves: int = 0
    # Checks
    checks: int = 0
    nonchecks: int = 0
    # Captures
    early_captures: int = 0
    mid_captures: int = 0
    late_captures: int = 0
    extreme_captures: int = 0
    # Capture distances
    capture_distance: list[int] = field(default_factory=[0 for _ in range(0, 8)].copy)
    noncapture_distance: list[int] = field(default_factory=[0 for _ in range(0, 8)].copy)
    # Game length
    game_length: list[int] = field(default_factory=[0 for _ in range(1024)].copy)
    short_games: int = 0
    medium_games: int = 0
    long_games: int = 0
    extreme_games: int = 0
    # Queen trades
    no_queens: list[int] = field(default_factory=[0 for _ in range(1024)].copy)
    total_trades: int = 0
    early_trades: int = 0
    mid_trades: int = 0
    late_trades: int = 0
    # Material imbalance
    num_QvRR: int = 0
    num_RRvQ: int = 0
    num_Qv3minor: int = 0
    num_3minorvQ: int = 0
    num_win_ahead: int = 0
    num_win_equal: int = 0
    num_win_behind: int = 0
    final_material: list[int] = field(default_factory=[0 for _ in range(207)].copy)
    # Pawn pushes
    early_pawn_pushes: list[int] = field(default_factory=[0 for _ in range(0, 8)].copy)
    mid_pawn_pushes: list[int] = field(default_factory=[0 for _ in range(0, 8)].copy)
    late_pawn_pushes: list[int] = field(default_factory=[0 for _ in range(0, 8)].copy)
    total_pawn_pushes: int = 0
    total_pawn_pushes_towards_king: int = 0
    # Bishop/Rook/Queen threats
    num_rook_threats: int = 0
    num_bishop_threats: int = 0

    def add_capture(self, ply: int):
        self.total_captures += 1
        self.total_moves += 1
        if ply < 30:
            self.early_captures += 1
        elif ply < 50:
            self.mid_captures += 1
        elif ply < 70:
            self.late_captures += 1
        else:
            self.extreme_captures += 1

    def add_noncapture(self):
        self.total_noncaptures += 1
        self.total_moves += 1

    def add_pawn_push(self, ply: int, to: chess.Square, side: chess.Color, enemy_ksq: chess.Square):
        self.total_pawn_pushes += 1

        to_rank = chess.square_rank(to)
        rank = to_rank if side == chess.WHITE else 7 - to_rank

        if ply < 40:
            self.early_pawn_pushes[rank] += 1
        elif ply < 60:
            self.mid_pawn_pushes[rank] += 1
        else:
            self.late_pawn_pushes[rank] += 1

        # Towards enemy king
        dx: int = chess.square_file(to) - chess.square_file(enemy_ksq)
        if abs(dx) <= 1:
            self.total_pawn_pushes_towards_king += 1

    def finish_game(self, ply: int):
        self.game_length[ply] += 1
        if ply < 80:
            self.short_games += 1
        elif ply < 100:
            self.medium_games += 1
        elif ply < 140:
            self.long_games += 1
        else:
            self.extreme_games += 1

    def queens_off(self, ply: int):
        self.no_queens[ply] += 1
        self.total_trades += 1
        if ply < 40:
            self.early_trades += 1
        elif ply < 60:
            self.mid_trades += 1
        else:
            self.late_trades += 1


def is_valid(stats: Stats) -> bool:
    """
    Check that the statistics are consistent
    """
    if stats.num_games != stats.num_wins + stats.num_draws + stats.num_losses:
        return False

    if stats.total_moves != stats.total_captures + stats.total_noncaptures:
        return False

    if stats.total_moves != stats.checks + stats.nonchecks:
        return False

    if stats.num_games != stats.num_wins + stats.num_draws + stats.num_losses:
        return False

    if stats.num_wins != stats.num_win_ahead + stats.num_win_equal + stats.num_win_behind:
        return False

    if stats.num_games != stats.short_games + stats.medium_games + stats.long_games + stats.extreme_games:
        return False

    if stats.total_captures != stats.early_captures + stats.mid_captures + stats.late_captures + stats.extreme_captures:
        return False

    if stats.early_pawn_pushes[0] > 0 or stats.early_pawn_pushes[1] > 0:
        return False

    if stats.mid_pawn_pushes[0] > 0 or stats.mid_pawn_pushes[1] > 0:
        return False

    if stats.late_pawn_pushes[0] > 0 or stats.late_pawn_pushes[1] > 0:
        return False
    
    if stats.total_pawn_pushes_towards_king > stats.total_pawn_pushes:
        return False

    return True


def get_aggression_score(stats: Stats, verbose: bool = False) -> float:
    """
    Score a player's aggressive play
    """
    def feature_game_length(stats: Stats) -> float:
        """
        Shorter games are more aggressive
        """
        return (0.6 * stats.short_games + 0.25 * stats.medium_games + 0.15 * stats.long_games) / stats.num_games / 0.6

    def feature_capture_early(stats: Stats) -> float:
        """
        Capturing early might favour quick attacks over slow positional play
        """
        return (0.6 * stats.early_captures + 0.25 * stats.mid_captures + 0.15 * stats.late_captures) / stats.total_captures / 0.6

    def feature_capture_near_king(stats: Stats) -> float:
        """
        Playing near the opponent's king is threatening
        """
        weights: list[int] = [0, 8, 4, 2, 1, 0, 0, 0]
        score: float = sum(weights[dist] * frequency for dist, frequency in enumerate(stats.capture_distance))
        max_score: float = max(weights) * stats.total_captures
        return score / max_score

    def feature_move_near_king(stats: Stats) -> float:
        """
        Moving near the opponent's king is threatening
        """
        weights: list[int] = [0, 8, 4, 2, 1, 0, 0, 0]
        score: float = sum(weights[dist] * frequency for dist, frequency in enumerate(stats.noncapture_distance))
        max_score: float = max(weights) * stats.total_noncaptures
        return score / max_score

    def feature_castle_opposite(stats: Stats) -> float:
        """
        Castling opposite more easily puts our rooks on the same side as the opponent's king
        """
        if stats.castle_opposite + stats.castle_same == 0:
            return 0.0
        return stats.castle_opposite / (stats.castle_opposite + stats.castle_same)

    def feature_sacrifices(stats: Stats) -> float:
        """
        Sacrifices
        """
        return 0.0

    def feature_push_pawns(stats: Stats) -> float:
        """
        Push pawns earlier
        """
        if stats.total_pawn_pushes == 0:
            return 0.0

        weights: list[int] = [0, 0, 1, 1, 2, 4, 8, 16]
        total_early_moves: int = sum(min(idx, 40) * freq for idx, freq in enumerate(stats.game_length))
        total_score: float = sum(weights[i] * stats.early_pawn_pushes[i] for i in range(2, 8))
        max_total_score: float = sum(weights[3:]) / len(weights[3:]) * total_early_moves
        return total_score / max_total_score

    def feature_checks(stats: Stats) -> float:
        """
        Checking moves
        """
        if stats.total_moves == 0:
            return 0.0

        return stats.checks / stats.total_moves

    def feature_wins_behind(stats: Stats) -> float:
        """
        Winning while behind on material might indicate sacrifices
        """
        if stats.num_wins == 0:
            return 0.0

        return stats.num_win_behind / stats.num_wins

    def feature_capture_frequency(stats: Stats) -> float:
        """
        Frequency of capturing moves
        """
        if stats.total_moves == 0:
            return 0.0

        return stats.total_captures / stats.total_moves

    def feature_push_pawn_towards_king(stats: Stats) -> float:
        """
        Pushing pawns towards the enemy king
        """
        if stats.total_pawn_pushes == 0:
            return 0.0

        return stats.total_pawn_pushes_towards_king / stats.total_pawn_pushes

    def feature_rook_threats(stats: Stats) -> float:
        """
        Rooks and Queens on the same rank or file as the enemy king
        """
        if stats.total_moves == 0:
            return 0.0

        return stats.num_rook_threats / stats.total_moves

    def feature_bishop_threats(stats: Stats) -> float:
        """
        Bishops and Queens on the same rank or file as the enemy king
        """
        if stats.total_moves == 0:
            return 0.0

        return stats.num_bishop_threats / stats.total_moves

    if stats.num_games == 0:
        return None

    features: list[float] = [
        (4.0,  "Game length", feature_game_length),
        (2.0,  "Capture early", feature_capture_early),
        (4.0,  "Capture near king", feature_capture_near_king),
        (2.0,  "Move near king", feature_move_near_king),
        (0.2,  "Castle opposite", feature_castle_opposite),
        (1.0,  "Push pawns", feature_push_pawns),
        (5.0,  "Checks", feature_checks),
        (5.0,  "Win while behind", feature_wins_behind),
        (5.0,  "Capture frequency", feature_capture_frequency),
        (4.0,  "Push pawns towards king", feature_push_pawn_towards_king),
        (4.0,  "Rook/Queen threats on king", feature_rook_threats),
        (4.0,  "Bishop/Queen threats on king", feature_bishop_threats),
        # (0.0,  "Sacrifices", feature_sacrifices),
    ]

    score: float = 0.0
    for weight, name, func in features:
        value: float = func(stats)

        assert(0.0 <= value and value <= 1.0)
        score += weight * value
    scaled = score / sum([weight for weight, _, _ in features])

    scaled = min(1.0, 2.0 * scaled)

    assert(0.0 <= scaled and scaled <= 1.0)

    if verbose:
        for weight, name, func in features:
            value: float = func(stats)
            weighted: float = weight * value
            contribution: float = 100 * weighted / score if score != 0.0 else 0.0
            print(f"Aggression value {value:.3f} weight {weight:.1f} weighted {weighted:.3f} contribution {contribution:.2f}% name {name}")

    return scaled


def get_positional_score(stats: Stats, verbose: bool = False) -> float:
    """
    Score a player's positional play
    """
    def feature_game_length(stats: Stats) -> float:
        """
        Longer games are probably more positional
        """
        return (0.6 * stats.long_games + 0.25 * stats.medium_games + 0.15 * stats.short_games) / stats.num_games / 0.6

    def feature_capture_early(stats: Stats) -> float:
        """
        Capturing early means less pieces to manoeuvre with
        """
        return (0.6 * stats.late_captures + 0.25 * stats.mid_captures + 0.15 * stats.early_captures) / stats.total_captures / 0.6

    if stats.num_games == 0:
        return None

    features: list[float] = [
        (2.0, "Game length", feature_game_length),
        (1.0, "Capture early", feature_capture_early),
    ]

    score: float = 0.0
    for weight, name, func in features:
        value: float = func(stats)

        assert(0.0 <= value and value <= 1.0)
        score += weight * value
    scaled = score / sum([weight for weight, _, _ in features])

    assert(0.0 <= scaled and scaled <= 1.0)

    if verbose:
        for weight, name, func in features:
            value: float = func(stats)
            weighted: float = weight * value
            contribution: float = 100 * weighted / score if score != 0.0 else 0.0
            print(f"Positional value {value:.3f} weight {weight:.1f} weighted {weighted:.3f} contribution {contribution:.2f}% name {name}")

    return scaled


def get_pawn_pusher_score(stats: Stats, verbose: bool = False) -> float:
    """
    Get a player's pawn pusher score
    """
    def feature_placeholder(stats: Stats) -> float:
        """
        Placeholder
        """
        return 0.0

    if stats.num_games == 0:
        return None

    features: list[float] = [
        (1.0, "Placeholder", feature_placeholder),
    ]

    score: float = 0.0
    for weight, name, func in features:
        value: float = func(stats)

        assert(0.0 <= value and value <= 1.0)
        score += weight * value
    scaled = score / sum([weight for weight, _, _ in features])

    assert(0.0 <= scaled and scaled <= 1.0)

    if verbose:
        for weight, name, func in features:
            value: float = func(stats)
            weighted: float = weight * value
            contribution: float = 100 * weighted / score if score != 0.0 else 0.0
            print(f"Pawn pusher value {value:.3f} weight {weight:.1f} weighted {weighted:.3f} contribution {contribution:.2f}% name {name}")

    return scaled


def get_material_score(board: chess.Board, side: chess.Color) -> int:
    """
    Basic material count
    """
    queens  = len(board.pieces(chess.QUEEN,  side))
    rooks   = len(board.pieces(chess.ROOK,   side))
    bishops = len(board.pieces(chess.BISHOP, side))
    knights = len(board.pieces(chess.KNIGHT, side))
    pawns   = len(board.pieces(chess.PAWN,   side))
    return 1 * pawns + 3 * knights + 3 * bishops + 5 * rooks + 9 * queens


def print_text(stats: Stats):
    if stats.num_games == 0:
        return

    print("Results:")
    print(f"Wins   {stats.num_wins:>5} ({100*stats.num_wins/stats.num_games:.1f}%)")
    print(f"Draws  {stats.num_draws:>5} ({100*stats.num_draws/stats.num_games:.1f}%)")
    print(f"Losses {stats.num_losses:>5} ({100*stats.num_losses/stats.num_games:.1f}%)")
    print(f"Total  {stats.num_games:>5}")
    print("")

    print("Castling:")
    if stats.castle_king + stats.castle_queen > 0:
        print(f"King   {stats.castle_king:>5} ({100*stats.castle_king/(stats.castle_king + stats.castle_queen):.1f}%)")
        print(f"Queen  {stats.castle_queen:>5} ({100*stats.castle_queen/(stats.castle_king + stats.castle_queen):.1f}%)")
        print(f"Total  {stats.castle_king + stats.castle_queen:>5} ({100*(stats.castle_king + stats.castle_queen)/stats.num_games:.1f}% of games)")
    print("")

    print("Castle side:")
    if stats.castle_same + stats.castle_opposite > 0:
        print(f"Same     {stats.castle_same:>5} ({100*stats.castle_same/(stats.castle_same + stats.castle_opposite):.1f}%)")
        print(f"Opposite {stats.castle_opposite:>5} ({100*stats.castle_opposite/(stats.castle_same + stats.castle_opposite):.1f}%)")
        print(f"Total    {stats.castle_opposite + stats.castle_same:>5}")
    print("")

    print("Move type:")
    print(f"captures {stats.total_captures:>6} ({100*stats.total_captures/stats.total_moves:.1f}%)")
    print(f"normal   {stats.total_noncaptures:>6} ({100*stats.total_noncaptures/stats.total_moves:.1f}%)")
    print(f"Total    {stats.total_noncaptures + stats.total_captures:>6}")
    print("")

    print("Checking:")
    print(f"checks    {stats.checks:>6} ({100*stats.checks/(stats.checks + stats.nonchecks):.1f}%)")
    print(f"nonchecks {stats.nonchecks:>6} ({100*stats.nonchecks/(stats.checks + stats.nonchecks):.1f}%)")
    print(f"Total     {stats.checks + stats.nonchecks:>6}")
    print("")

    print("Captures:")
    print(f"Early   {stats.early_captures:>5} ({100*stats.early_captures/stats.total_captures:.1f}%)")
    print(f"Mid     {stats.mid_captures:>5} ({100*stats.mid_captures/stats.total_captures:.1f}%)")
    print(f"Late    {stats.late_captures:>5} ({100*stats.late_captures/stats.total_captures:.1f}%)")
    print(f"Extreme {stats.extreme_captures:>5} ({100*stats.extreme_captures/stats.total_captures:.1f}%)")
    print(f"Total   {stats.total_captures:>5}")
    print("")

    print("Queen trades:")
    if stats.total_trades > 0:
        print(f"Early {stats.early_trades:>5} ({100*stats.early_trades/stats.total_trades:.1f}%)")
        print(f"Mid   {stats.mid_trades:>5} ({100*stats.mid_trades/stats.total_trades:.1f}%)")
        print(f"Late  {stats.late_trades:>5} ({100*stats.late_trades/stats.total_trades:.1f}%)")
        print(f"Total {stats.total_trades:>5} ({100*stats.total_trades/stats.num_games:.1f}% of games)")
    print("")

    print("Game length:")
    print(f"Early   {stats.short_games:>5} ({100*stats.short_games/stats.num_games:.1f}%)")
    print(f"Mid     {stats.medium_games:>5} ({100*stats.medium_games/stats.num_games:.1f}%)")
    print(f"Late    {stats.long_games:>5} ({100*stats.long_games/stats.num_games:.1f}%)")
    print(f"Extreme {stats.extreme_games:>5} ({100*stats.extreme_games/stats.num_games:.1f}%)")
    print(f"Total   {stats.num_games:>5} ({100*stats.num_games/stats.num_games:.1f}% of games)")
    print("")

    print("Material imbalance:")
    print(f"Q vs RR      {stats.num_QvRR:>5} ({100*stats.num_QvRR/stats.num_games:.1f}%)")
    print(f"RR vs Q      {stats.num_RRvQ:>5} ({100*stats.num_RRvQ/stats.num_games:.1f}%)")
    print(f"Q vs 3Minor  {stats.num_Qv3minor:>5} ({100*stats.num_Qv3minor/stats.num_games:.1f}%)")
    print(f"3Minor vs Q  {stats.num_3minorvQ:>5} ({100*stats.num_3minorvQ/stats.num_games:.1f}%)")
    print("")

    print("Capture distance to enemy king:")
    gg: int = 0
    for idx, amount in enumerate(stats.capture_distance[1:]):
        gg += idx * amount
        print(f"dist {idx + 1} captures {amount:>6} ({100*amount/stats.total_captures:.1f}%)")
    print(f"Mean: {gg / stats.total_captures:.3f}")
    print("")

    print("Noncapture distance to enemy king:")
    gg: int = 0
    for idx, amount in enumerate(stats.noncapture_distance[1:]):
        gg += idx * amount
        print(f"dist {idx + 1} noncaptures {amount:>6} ({100*amount/stats.total_noncaptures:.1f}%)")
    print(f"Mean: {gg / stats.total_noncaptures:.3f}")
    print("")

    print("Material imbalance on gameover - win:")
    if stats.num_wins:
        print(f"ahead:  {stats.num_win_ahead:>5} ({100 * stats.num_win_ahead / stats.num_wins:.1f}%)")
        print(f"equal:  {stats.num_win_equal:>5} ({100 * stats.num_win_equal / stats.num_wins:.1f}%)")
        print(f"behind: {stats.num_win_behind:>5} ({100 * stats.num_win_behind / stats.num_wins:.1f}%)")
    print("")

    print("Pawn pushes by rank:")
    if stats.total_pawn_pushes > 0:
        gg: (int, int, int) = (0, 0, 0)
        print("Rank    Early                Mid                  Late")
        for idx in range(2, 8):
            gg = (gg[0] + stats.early_pawn_pushes[idx], gg[1] + stats.mid_pawn_pushes[idx], gg[2] + stats.late_pawn_pushes[idx])
            print(f"{idx + 1}", end="")
            print(f"       {stats.early_pawn_pushes[idx]:>6} ({100*stats.early_pawn_pushes[idx]/stats.total_pawn_pushes:4.1f}%)", end="")
            print(f"       {stats.mid_pawn_pushes[idx]:>6} ({100*stats.mid_pawn_pushes[idx]/stats.total_pawn_pushes:4.1f}%)", end="")
            print(f"       {stats.late_pawn_pushes[idx]:>6} ({100*stats.late_pawn_pushes[idx]/stats.total_pawn_pushes:4.1f}%)", end="")
            print("")
        print(f"Mean:    {gg[0] / stats.total_moves:.3f}                {gg[1] / stats.total_moves:.3f}                {gg[2] / stats.total_moves:.3f}")
        print("")

    print("Total material remaining on gameover (1,3,3,5,9):")
    mean: int = 0
    less_equal_5: int = 0
    less_equal_10: int = 0
    less_equal_15: int = 0
    less_equal_20: int = 0
    less_equal_30: int = 0
    less_equal_40: int = 0
    less_equal_60: int = 0
    less_equal_80: int = 0
    for idx, freq in enumerate(stats.final_material):
        mean += idx * freq
        if idx <= 5:
            less_equal_5 += freq
        if idx <= 10:
            less_equal_10 += freq
        if idx <= 15:
            less_equal_15 += freq
        if idx <= 20:
            less_equal_20 += freq
        if idx <= 30:
            less_equal_30 += freq
        if idx <= 40:
            less_equal_40 += freq
        if idx <= 60:
            less_equal_60 += freq
        if idx <= 80:
            less_equal_80 += freq
    mean /= stats.num_games
    print(f"<= 5:    {less_equal_5:>5} ({100 * less_equal_5 / stats.num_games:.1f}%)")
    print(f"<= 10:   {less_equal_10:>5} ({100 * less_equal_10 / stats.num_games:.1f}%)")
    print(f"<= 15:   {less_equal_15:>5} ({100 * less_equal_15 / stats.num_games:.1f}%)")
    print(f"<= 20:   {less_equal_20:>5} ({100 * less_equal_20 / stats.num_games:.1f}%)")
    print(f"<= 30:   {less_equal_30:>5} ({100 * less_equal_30 / stats.num_games:.1f}%)")
    print(f"<= 40:   {less_equal_40:>5} ({100 * less_equal_40 / stats.num_games:.1f}%)")
    print(f"<= 60:   {less_equal_60:>5} ({100 * less_equal_60 / stats.num_games:.1f}%)")
    print(f"<= 80:   {less_equal_80:>5} ({100 * less_equal_80 / stats.num_games:.1f}%)")
    print(f"mean:     {mean:.1f}")


def analyse_game(game: chess.pgn.Game, side: chess.Color, stats: Stats) -> Stats:
    """
    Analyse the game given and return statistics
    """
    if game.headers["Result"] not in ["1-0", "0-1", "1/2-1/2"]:
        raise RuntimeError(f'Invalid game result {game.headers["Result"]}')

    fen = game.headers["FEN"] if "FEN" in game.headers else "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    board = chess.Board(fen)
    us_castled = 0
    them_castled = 0
    queens_gone = False
    ply = 0
    # Material imbalance
    has_QvRR = False
    has_RRvQ = False
    has_Qv3minor = False
    has_3minorvQ = False

    # Look at the moves
    for idx, move in enumerate(game.mainline_moves()):
        num_queens_white = len(board.pieces(chess.QUEEN, chess.WHITE))
        num_queens_black = len(board.pieces(chess.QUEEN, chess.BLACK))
        piece = board.piece_type_at(move.from_square)
        enemy_ksq = board.king(not board.turn)

        # Traded queens?
        if queens_gone == False and num_queens_white == 0 and num_queens_black == 0:
            stats.queens_off(idx)
            queens_gone = True

        if board.turn == side:
            num_queens_us = num_queens_white if side == chess.WHITE else num_queens_black
            num_queens_them = num_queens_black if side == chess.WHITE else num_queens_white
            num_rooks_us = len(board.pieces(chess.ROOK, side))
            num_rooks_them = len(board.pieces(chess.ROOK, not side))
            num_minor_us = len(board.pieces(chess.KNIGHT, side)) + len(board.pieces(chess.BISHOP, side))
            num_minor_them = len(board.pieces(chess.KNIGHT, not side)) + len(board.pieces(chess.BISHOP, not side))
            dist_to_enemy_king: int = chess.square_distance(move.to_square, board.king(not side))

            # Material imbalance
            if num_queens_us == 1 and num_queens_them == 0 and num_rooks_us == 0 and num_rooks_them == 2:
                has_QvRR = True
            if num_queens_us == 0 and num_queens_them == 1 and num_rooks_us == 2 and num_rooks_them == 0:
                has_RRvQ = True
            if num_queens_us == 1 and num_queens_them == 0 and num_minor_us == 0 and num_minor_them == 3:
                has_Qv3minor = True
            if num_queens_us == 0 and num_queens_them == 1 and num_minor_us == 3 and num_minor_them == 0:
                has_3minorvQ = True

            # Castling
            if board.is_kingside_castling(move):
                stats.castle_king += 1
                us_castled = 1
            elif board.is_queenside_castling(move):
                stats.castle_queen += 1
                us_castled = 2

            # Threats
            dx: int = chess.square_file(enemy_ksq) - chess.square_file(move.to_square)
            dy: int = chess.square_rank(enemy_ksq) - chess.square_rank(move.to_square)
            if piece in [chess.ROOK, chess.QUEEN]:
                if abs(dx) <= 1 or abs(dy) <= 1:
                    stats.num_rook_threats += 1
            if piece in [chess.BISHOP, chess.QUEEN]:
                if abs(abs(dx) - abs(dy)) <= 1:
                    stats.num_bishop_threats += 1

            # Move types
            if board.is_capture(move):
                stats.add_capture(ply)
                stats.capture_distance[dist_to_enemy_king] += 1
            else:
                stats.add_noncapture()
                stats.noncapture_distance[dist_to_enemy_king] += 1

            if board.piece_type_at(move.from_square) == chess.PAWN:
                stats.add_pawn_push(ply, move.to_square, board.turn, board.king(not board.turn))

        else:
            # Castling
            if board.is_kingside_castling(move):
                them_castled = 1
            elif board.is_queenside_castling(move):
                them_castled = 2

        board.push(move)

        if board.turn != side:
            if board.is_check():
                stats.checks += 1
            else:
                stats.nonchecks += 1

        ply += 1

    # Material imbalance
    if has_QvRR:
       stats.num_QvRR += 1
    if has_RRvQ:
       stats.num_RRvQ += 1
    if has_Qv3minor:
       stats.num_Qv3minor += 1
    if has_3minorvQ:
       stats.num_3minorvQ += 1

    # No one castled
    if us_castled == 0 and them_castled == 0:
        pass
    # They castled, we didn't
    elif us_castled == 0 and them_castled != 0:
        pass
    # We castled, they didn't
    elif us_castled != 0 and them_castled == 0:
        pass
    # Both castled king side
    elif us_castled == 1 and them_castled == 1:
        stats.castle_same += 1
    # Both castled queen side
    elif us_castled == 2 and them_castled == 2:
        stats.castle_same += 1
    # Us king, them queen
    elif us_castled == 1 and them_castled == 2:
        stats.castle_opposite += 1
    # Us queen, them king
    elif us_castled == 2 and them_castled == 1:
        stats.castle_opposite += 1
    else:
        raise RuntimeError(f"Invalid castling flags {us_castled} and {them_castled}")

    stats.finish_game(ply)
    stats.num_games += 1

    material_us: int = get_material_score(board, side)
    material_them: int = get_material_score(board, not side)
    is_material_ahead: bool = material_us > material_them
    is_material_equal: bool = material_us == material_them
    is_material_behind: bool = material_us < material_them
    if game.headers["Result"] == "1/2-1/2":
        stats.num_draws += 1
    elif game.headers["Result"] == "1-0":
        if side == chess.WHITE:
            stats.num_wins += 1
            stats.num_win_ahead += is_material_ahead
            stats.num_win_equal += is_material_equal
            stats.num_win_behind += is_material_behind
        else:
            stats.num_losses += 1
    elif game.headers["Result"] == "0-1":
        if side == chess.BLACK:
            stats.num_wins += 1
            stats.num_win_ahead += is_material_ahead
            stats.num_win_equal += is_material_equal
            stats.num_win_behind += is_material_behind
        else:
            stats.num_losses += 1
    stats.final_material[material_us + material_them] += 1


def analyse_pgn(path: str, filters, games: int) -> (Stats, Stats):
    """
    Analyse every game in a .pgn and return separate statistics
    """
    count: int = 0

    with open(path) as pgn:
        while True:
            game = chess.pgn.read_game(pgn)

            if game is None or game.headers is None:
                break

            if "Result" not in game.headers or game.headers["Result"] not in ["1-0", "0-1", "1/2-1/2"]:
                continue

            if "White" not in game.headers or "Black" not in game.headers:
                continue

            for _, stats, conditions in filters:
                for side, player in [(chess.WHITE, game.headers["White"]), (chess.BLACK, game.headers["Black"])]:
                    is_right = conditions(player, side, game)
                    if not is_right:
                        continue

                    analyse_game(game, side, stats)
                    count += 1
                    assert(is_valid(stats))

                    if games and count >= games:
                        return


def main():
    parser = argparse.ArgumentParser(description="Chess play style analysis")
    parser.add_argument("--pgn", type=str, help="Path to the pgn")
    parser.add_argument("--player", type=str, help="Name of the player")
    parser.add_argument("--games", type=int, help="Max number of games to analyse")
    parser.add_argument("--verbose", action="store_true", help="Verbose output")
    parser.add_argument("--white", action="store_true", help="Analyse games - white")
    parser.add_argument("--black", action="store_true", help="Analyse games - black")
    parser.add_argument("--wins", action="store_true", help="Analyse games - wins")
    parser.add_argument("--losses", action="store_true", help="Analyse games - losses")
    parser.add_argument("--draws", action="store_true", help="Analyse games - draws")
    parser.add_argument("--all", action="store_true", help="Analyse games - all")
    args = parser.parse_args()

    filters = []
    # filters = [
    #     # ("All games",     Stats(), lambda player, _, __ : player == args.player),
    #     ("White",               Stats(), lambda player, side, _    : player == args.player and side == chess.WHITE),
    #     ("Black",               Stats(), lambda player, side, _    : player == args.player and side == chess.BLACK),
    #     # ("Wins only",     Stats(), lambda player, side, game : player == args.player and ((side == chess.WHITE and game.headers["Result"] == "1-0") or (side == chess.BLACK and game.headers["Result"] == "0-1"))),
    #     # ("Losses only",   Stats(), lambda player, side, game : player == args.player and ((side == chess.WHITE and game.headers["Result"] == "0-1") or (side == chess.BLACK and game.headers["Result"] == "1-0"))),
    #     # ("White (wins only)",   Stats(), lambda player, side, game : player == args.player and side == chess.WHITE and game.headers["Result"] == "1-0"),
    #     # ("White (losses only)", Stats(), lambda player, side, game : player == args.player and side == chess.WHITE and game.headers["Result"] == "0-1"),
    #     # ("White (draws only)",  Stats(), lambda player, side, game : player == args.player and side == chess.WHITE and game.headers["Result"] == "1/2-1/2"),
    #     # ("Black (wins only)",   Stats(), lambda player, side, game : player == args.player and side == chess.BLACK and game.headers["Result"] == "0-1"),
    #     # ("Black (losses only)", Stats(), lambda player, side, game : player == args.player and side == chess.BLACK and game.headers["Result"] == "1-0"),
    #     # ("Black (draws only)",  Stats(), lambda player, side, game : player == args.player and side == chess.BLACK and game.headers["Result"] == "1/2-1/2"),
    # ]

    if args.all:
        filters.append(("All games", Stats(), lambda player, side, _ : player == args.player))
    if args.white:
        filters.append(("White", Stats(), lambda player, side, _ : player == args.player and side == chess.WHITE))
    if args.black:
        filters.append(("Black", Stats(), lambda player, side, _ : player == args.player and side == chess.BLACK))
    if args.wins:
        filters.append((
            "Wins only",
            Stats(),
            lambda player, side, game : player == args.player and ((side == chess.WHITE and game.headers["Result"] == "1-0") or (side == chess.BLACK and game.headers["Result"] == "0-1"))
        ))
    if args.losses:
        filters.append((
            "Losses only",
            Stats(),
            lambda player, side, game : player == args.player and ((side == chess.WHITE and game.headers["Result"] == "0-1") or (side == chess.BLACK and game.headers["Result"] == "1-0"))
        ))
    if args.draws:
        filters.append((
            "Draws only",
            Stats(),
            lambda player, side, game : player == args.player and game.headers["Result"] == "1/2-1/2"
        ))

    styles = [
        ("Aggressive ", get_aggression_score),
        ("Positional ", get_positional_score),
        ("Pawn pusher", get_pawn_pusher_score),
    ]

    analyse_pgn(args.pgn, filters, args.games)

    for name, stats, _ in filters:
        if args.verbose and len(filters) > 1:
            print("------------------------------------------------")
            print("")
            print("")

        print(f"Filter: {name}")
        if stats.num_games <= 0:
            print("No games found")
            print("")
            print("")
            continue

        print("Score:")
        print(f"- games  {stats.num_games:>5}")
        print(f"- wins   {stats.num_wins:>5} ({100*stats.num_wins/stats.num_games:.2f}%)")
        print(f"- draws  {stats.num_draws:>5} ({100*stats.num_draws/stats.num_games:.2f}%)")
        print(f"- losses {stats.num_losses:>5} ({100*stats.num_losses/stats.num_games:.2f}%)")

        print("Styles:")
        for name, func in styles:
            score: float = func(stats, args.verbose)
            if score == None:
                print(f"- {name}  None")
            else:
                print(f"- {name}  {score:.3f}")

        # if args.verbose:
        #     print_text(stats)

        print("")

        if args.verbose and len(filters) > 1:
            print("")


if __name__ == '__main__':
    main()
