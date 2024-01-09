library(readr)
library(here)
library(stringr)
library(tibble)

input <- readr::read_lines(here("input"))

# Create a tibble with columns: game, round, color, amount
# Each row is the amount of dice of a color in a round of a game
parse_game <- function(text) {
  id <- str_extract(text, "^Game (\\d+):", 1) |>
    as.integer()

  rounds <- text |>
    str_extract("^Game \\d+: (.*)", 1) |>
    str_split_1(";") |>
    imap(function(x, i) {
      dice_groups <- str_split_1(x, ",\\s") |>
          str_trim()

      dice_groups |>
        map(function(y) {
          str_split_1(y, "\\s") |>
            set_names(c("amount", "color")) |>
            as_tibble_row()
        }) |>
        list_rbind() |>
        mutate(
          round = i,
          amount = as.integer(amount)
        )
    }) |>
    list_rbind() |>
    mutate(game = id)
}

data <- input |>
  map(parse_game) |>
  list_rbind()


## Part 1 ########################################################

# Create our source of truth bag
bag <- tibble(
  color = c("red", "green", "blue"),
  amount = c(12L, 13L, 14L)
)

# Initialize a vector to hold the games that are broken
broken_games <- integer()

# If higher value for color in one round than in bag, drop the round's game
data |>
  pwalk(function(amount, color, round, game) {
    if (amount > bag$amount[bag$color == color]) {
      broken_games <<- c(broken_games, game)
    }
  })

# Filter out the broken games, then sum the unique game ids
data |>
  filter(
    !(game %in% broken_games)
  ) |>
  pull(game) |>
  unique() |>
  sum()

## Part 2 ########################################################

data |>
  summarise(
    .by = c("game", "color"),
    amount = max(amount)
  ) |>
  summarise(
    .by = "game",
    amount = prod(amount)
  ) |>
  pull(amount) |>
  sum()
