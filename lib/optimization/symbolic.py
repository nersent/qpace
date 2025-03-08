from typing import TypedDict


class SymbolicOptimizerOptions(TypedDict):
    save_solutions: bool
    log_progress: bool
    num_parents_mating: int
    sol_per_pop: int
    parent_selection_type: str
    mutation_probability: float
    allow_duplicate_genes: bool
    k_tournament: int
    keep_elitism: int
    mutation_by_replacement: bool
    mutation_percent_genes: int
    initial_population: list
    generations: int


class SymbolicOptimizer:
    DEFAULT_OPTIONS: SymbolicOptimizerOptions = {
        "log_progress": True,
        "save_solutions": False,
        "sol_per_pop": 2,
        "parent_selection_type": "sss",
        "mutation_probability": 0.25,
        "num_parents_mating": 2,
        "allow_duplicate_genes": False,
        "k_tournament": 3,
        "keep_elitism": 1,
        "crossover_type": "single_point",
        "mutation_type": "random",
        "mutation_by_replacement": True,
        "mutation_percent_genes": 10,
        "initial_population": None,
        "generations": 1,
    }
