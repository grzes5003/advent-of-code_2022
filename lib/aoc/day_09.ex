defmodule AOC.Day09 do
  @directions [:down, :up, :left, :right]

  def get_chunks(list) do
    list
    |> Enum.chunk_by(fn x -> x != "" end)
    |> Enum.reject(fn x -> x == [""] end)
  end

  def direction("U"), do: :up
  def direction("D"), do: :down
  def direction("L"), do: :left
  def direction("R"), do: :right

  def next({x,y}, dir) do
    case dir do
      :right -> {x, y-1}
      :left -> {x, y+1}
      :down -> {x-1, y}
      :up -> {x+1, y}
    end
  end

  def move_t({x_t, y_t}, {x_h, y_h}) do
    {x,y} = {x_h-x_t, y_h-y_t}
    case {x,y} do
      {x,y} when abs(x) > 1 -> {sign(x) + x_t, y + y_t}
      {x,y} when abs(y) > 1 -> {x + x_t, sign(y) + y_t}
      _ -> {x_t, y_t}
    end
  end

  def sign(int) when int >= 0, do: 1
  def sign(int) when int < 0, do: -1

  def parse(list) do
    list
    |> Enum.map(fn line -> String.split(line, " ", trim: true) end)
    |> Enum.map(fn [a,b] -> {direction(a), String.to_integer(b)} end)
  end

  def move(_, _, [], visited), do: visited
  def move({x_t, y_t}, {x_h, y_h}, com, visited \\ MapSet.new()) do
    {dir, len} = com |> hd
    {x_h, y_h} = next({x_h, y_h}, dir)
    {x_t, y_t} = move_t({x_t, y_t}, {x_h, y_h})
    visited = MapSet.put(visited, {x_t,y_t})
    com = case len do
      1 -> com |> tl
      _ -> [ {dir, len-1} | (com |> tl)]
    end
    move({x_t, y_t}, {x_h, y_h}, com, visited)
  end

  def part01(args) do
    args
    |> parse
    |> (&(move({0,0}, {0,0}, &1))).()
    |> MapSet.size()
  end

  def part02(args) do
  end
end
