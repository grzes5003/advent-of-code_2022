defmodule AOC.Day04Test do
  use ExUnit.Case

  import AOC.Day04

  def input do
    ["2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8"]
  end

  test "part01" do
    assert part01(input()) == 2
  end

  test "part02" do
    assert part02(input()) == 4
  end
end
