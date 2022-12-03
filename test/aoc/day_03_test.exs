defmodule AOC.Day03Test do
  use ExUnit.Case

  import AOC.Day03

  def input do
    [
      "vJrwpWtwJgWrhcsFMMfFFhFp",
      "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
      "PmmdzqPrVvPwwTWBwg",
      "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
      "ttgJtRGJQctTZtZT",
      "CrZsJsPPZsGzwwsLwLmpwMDw"
    ]
  end

  test "part01" do
    assert part01(input()) == 157
  end

  test "part02" do
    assert part02(input()) == 70
  end
end
