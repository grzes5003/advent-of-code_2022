defmodule AOC.Day07 do

  defmodule Node do
    defstruct name: nil, type: nil, size: 0, uuid: nil
  end

  @commads [:cd, :ls, :up]
  @types [:dir, :file]

  def parse_command(line) do
    case String.split(line, " ", trim: true) do
      [_, _, val] -> {:cd, val}
      _ -> nil
    end
  end

  def parse_ls(line) do
    case String.split(line, " ", trim: true) do
      [a, b] when a == "dir" -> %Node{name: b, type: :dir, size: 0, uuid: UUID.uuid1()}
      [a, b] -> %Node{name: b, type: :file, size: String.to_integer(a), uuid: UUID.uuid1()}
    end
  end

  def parse_line(line) do
    case String.at(line, 0) do
      "$" -> parse_command(line)
      _ -> parse_ls(line)
    end
  end

  def tree(graph, _, []), do: graph
  def tree(graph, current, comms) when is_tuple(hd(comms)) do
    {_, name} = hd(comms)
    next =
      case name do
        ".." ->
          :digraph.in_neighbours(graph, current) |> hd
        val when val == current.name -> current
        val -> (
          with _ = [] <- :digraph.out_neighbours(graph, current) do
            tree(graph, current, [%Node{name: val, type: :dir, size: 0, uuid: UUID.uuid1()}  | comms])
          else list ->
            Enum.find(list ,fn node -> node.name == val end)
          end
        )
      end
    tree(graph, next, comms |> tl)
  end
  def tree(graph, current, comms) when is_struct(hd(comms)) do
    vertex = hd(comms)
    :digraph.add_vertex(graph, vertex)
    :digraph.add_edge(graph, current, vertex)

    tree(graph, current, tl(comms))
  end

  def dfs(graph, vertex, acc \\ []) do
    case vertex in acc do
      false -> acc = [vertex | acc]
        Enum.reduce(:digraph.out_neighbours(graph, vertex), acc, fn elem, acc ->
          dfs(graph, elem, acc)
        end)
      true -> acc
    end
  end

  def count(graph, limit) do
    :digraph.vertices(graph)
    |> Enum.filter(fn node -> node.type == :dir end)
    |> Enum.map(fn dir -> dfs(graph, dir) end)
    |> Enum.map(fn dfs ->
      Enum.map(dfs, fn node -> node.size end)
      |> Enum.reduce(fn node, acc -> acc + node end)
    end)
    |> Enum.filter(fn size -> size <= limit end)
    |> Enum.sum()
  end

  def smallest(graph, limit) do
    :digraph.vertices(graph)
    |> Enum.filter(fn node -> node.type == :dir end)
    |> Enum.map(fn dir -> dfs(graph, dir) end)
    |> Enum.map(fn dfs ->
      Enum.map(dfs, fn node -> node.size end)
      |> Enum.reduce(fn node, acc -> acc + node end)
    end)
    |> Enum.filter(fn size -> size >= limit end)
    |> Enum.min
  end

  def setup(args) do
    comms =
      args
      |> Enum.map(&parse_line/1)
      |> Enum.filter(fn com -> com != nil end)

    root = %Node{name: "/", type: :dir, size: 0}
    graph = :digraph.new()
    :digraph.add_vertex(graph, root)

    tree(graph, root, comms)
  end

  def part01(args) do
    setup(args)
    |> count(100_000)
  end

  def part02(args) do
    graph = setup(args)
    total_used = graph |>  dfs(%Node{name: "/", type: :dir, size: 0})
    |> Enum.map(fn node -> node.size end)
    |> Enum.reduce(fn node, acc -> acc + node end)

    graph |> smallest(30_000_000 - (70_000_000 - total_used))
  end
end
