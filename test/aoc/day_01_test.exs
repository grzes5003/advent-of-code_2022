defmodule AOC.Day01Test do
  use ExUnit.Case

  import AOC.Day01

  def a do
    [[1, 2], [5, 6]]
  end

  def input do
    [
      "1000", "2000", "3000", "",
      "4000", "",
      "5000", "6000", "",
      "7000", "8000", "9000", "",
      "10000"]
  end

  test "part01" do
    assert part01(input()) == 24000
  end
end
