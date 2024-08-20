library(tidyverse)
library(here)
library(purrr)

data <- read_file(here("2022/day_1/input/day_1.txt")) |>
  str_split_1("\n") |>
  as.numeric()

res <- list(
  c()
)
j <- 1
for (i in seq_along(data)) {
  if (!is.na(data[i])) {
    res[[j]] <- c(res[[j]], data[i])
  } else {
    j <- j + 1
    res[[j]] <- c(data[i])
  }
}

res |>
  map(na.exclude)
  
