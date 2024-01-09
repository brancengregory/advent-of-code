library(here)
library(readr)
library(stringr)
library(dplyr)
library(purrr)

input <- readr::read_delim(
  file = here("input"),
  delim = "\n",
  col_names = FALSE
)

# Part 1 ########################################################

data <- input |>
  mutate(
    digits = str_extract_all(X1, "\\d")
  ) |>
  mutate(
    first_digit = map_int(digits, ~ .x[1] |> as.numeric()),
    last_digit = map_int(digits, ~ .x[length(.x)] |> as.numeric()),
    full_number = first_digit * 10 + last_digit
  )

sum(data$full_number)

# Part 2 ########################################################

## Minimum number of names needed to represent all integers from 0 through 99
digit_names <- tibble(
  digit = c(1:9),
  name = c("one", "two", "three", "four", "five", "six", "seven", "eight", "nine")
)

extract_first_and_last <- function(string, patterns, replacements) {
  digit_locations <- str_locate_all(string, "\\d")

  if (length(digit_locations[[1]]) > 0) {
    digit_locations <- map2_dfr(digit_locations[[1]][, 1], digit_locations[[1]][, 2], ~{
      tibble(
        start = .x,
        end = .y,
        digit = str_sub(string, .x, .y),
        pattern = NA_character_
      )
    })
  } else {
    tibble(
      start = integer(),
      end = integer(),
      digit = character(),
      pattern = character()
    )
  }

  # Find all matches for each pattern and create a tibble for each
  matches_list <- map2(patterns, seq_along(patterns), ~{
    matches <- str_locate_all(string, fixed(.x))
    if (length(matches[[1]]) == 0) return(NULL) # skip if no matches

    tibble(
      start = matches[[1]][, 1],
      end = matches[[1]][, 2],
      digit = as.character(.y), # using the pattern's index as 'digit'
      pattern = .x
    )
  })

  # Combine all tibbles into one
  matches <- bind_rows(matches_list) |>
    bind_rows(digit_locations)

  if (nrow(matches) == 0) {
    return(string)
  }

  matches <- matches |>
    arrange(start)

  first_digit <- matches$digit[1]
  last_digit <- matches$digit[nrow(matches)]

  full_number <- as.integer(paste0(first_digit, last_digit))

  return(full_number)
}

data <- input |>
  mutate(
    full_number = map_int(
      X1,
      ~ extract_first_and_last(.x, digit_names$name, digit_names$digit)
    )
  )

sum(data$full_number)
