defmodule AOC.Day08 do

  @directions [:down, :up, :left, :right]

  def neg(dir) do
    case dir do
      :down -> :up
      :left -> :right
      :right -> :left
      :up -> :down
    end
  end

  def gen_matrix(n) do
    List.duplicate(List.duplicate(false, n), n)
  end

  def parse(list) do
    list
    |> Enum.map(fn x -> String.split(x, "", trim: true) end)
    |> Enum.map(fn x -> Enum.map(x, fn y -> String.to_integer(y) end) end)
  end

  def next({x,y}, dir) do
    case dir do
      :right -> {x, y + 1}
      :left -> {x, y-1}
      :down -> {x+1, y}
      :up -> {x-1, y}
    end
  end

  def row(ref, mask,  {x,y}, _, _) when length(ref) in [x,y] or -1 in [x,y], do: mask
  def row(ref, mask,  {x,y}, dir, max \\ -1) do
    n = length(ref)
    {x_prv, y_prv} = next({x,y}, neg(dir))
    value_prv =
      if Enum.all?([x_prv, y_prv], &(&1 >= 0 && &1 < n)), do:
        ref |> Enum.at(x_prv, []) |> Enum.at(y_prv, 0),
      else: -1

    value_prv = Enum.max([value_prv, max])

    value = ref
    |> Enum.at(x)
    |> Enum.at(y)

    mask = if value_prv < value do
      mask |> List.update_at(x, &(List.replace_at(&1, y, true)))
    else
      mask
    end
    row(ref, mask, next({x,y}, dir), dir, value_prv)
  end

  def row2(ref, len,  _, {x,y}, _) when length(ref) in [x,y] or -1 in [x,y], do: len
  def row2(ref, len, start, {x,y}, dir) do
    value = ref
    |> Enum.at(x)
    |> Enum.at(y)

    if start > value do
      row2(ref, len + 1, start, next({x,y}, dir), dir)
    else
      len + 1
    end
  end


  def visible(ref) do
      range = Enum.to_list(0..length(ref)-1)
      zeros = List.duplicate(0, length(ref))
      ns = List.duplicate(length(ref) - 1, length(ref))
      [
        {:right, Enum.zip(range, zeros)},
        {:left, Enum.zip(range, ns)},
        {:down, Enum.zip(zeros, range)},
        {:up, Enum.zip(ns, range)},
      ]
  end

  def rows(_, mask, [], _), do: mask
  def rows(ref, mask, next, dir) do
    coords = next |> hd
    mask = row(ref, mask, coords, dir)
    rows(ref, mask, next |> tl, dir)
  end

  def vis(_, mask, []), do: mask
  def vis(ref, mask, comms) do
    {dir, coords} = comms |> hd
    mask = rows(ref, mask, coords, dir)
    vis(ref, mask, comms |> tl)
  end

  def sum(mask) do
    mask
    |> Enum.map(fn list -> Enum.map(list, &(if &1, do: 1, else: 0)) end)
    |> Enum.map(fn list -> Enum.sum(list) end)
    |> Enum.sum
  end

  def visibility(ref, {x,y}) do
    start = ref
    |> Enum.at(x)
    |> Enum.at(y)

    @directions
    |> Enum.map(fn dir -> row2(ref, 0, start, next({x,y}, dir), dir) end)
    |> Enum.reduce(fn item, acc -> item * acc end)
  end

  def part01(args) do
    ref = parse(args)
    mask = gen_matrix(length(ref))
    vis(ref, mask, visible(ref))
    |> sum
  end

  def part02(args) do
    ref = parse(args)
    1..length(ref)-1
    |> Enum.map(fn x ->
      Enum.map(1..length(ref)-1, &({x,&1}))
    end)
    |> List.flatten
    |> Enum.map(&(visibility(ref, &1)))
    |> Enum.max
  end
end
