library(httr2)
library(fs)
library(readr)
library(stringr)
library(dplyr)
library(tibble)
library(purrr)

input <- readr::read_delim(
  file = "input.txt",
  delim = "\n",
  col_names = FALSE
)

input |>
  mutate(
    first = map_chr(X1, ~ str_extract(.x, "\\d")),
  )

test <- input |>
  pull(X1)

test |>
  pull()
