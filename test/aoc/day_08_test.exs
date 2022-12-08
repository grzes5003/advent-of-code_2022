defmodule AOC.Day08Test do
  use ExUnit.Case

  import AOC.Day08

  def input do
    [
      "30373",
      "25512",
      "65332",
      "33549",
      "35390"
    ]
  end

  test "part01" do
    assert part01(input()) == 21
  end

  test "part02" do
    assert part02(input()) == 8
  end
end
