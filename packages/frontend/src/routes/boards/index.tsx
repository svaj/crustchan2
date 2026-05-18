import type { TuonoRouteProps } from 'tuono'

interface BoardsListData {
  categories:[{
    id: String,
    name: String,
    sort_order: Number,
    created_at: String,
    boards: [Object],
  }]
  };

export default function Boards({ 
  isLoading,
  data,
}: TuonoRouteProps<BoardsListData>) {
  let listOrLoad = <>Loading...</>
  if (!isLoading && data) {
    listOrLoad =  
      <ul><p>Categories</p>
        {data.categories.map(category =>
          <li key={category.id.toString()}><h2>{category.name}</h2>
            <p>{JSON.stringify(category)}</p>
            <ul>{category.boards.map(board => <li>{JSON.stringify(board)}</li>)}</ul>
          </li>
        )}
      </ul>
  }
  return <div>
    <h1>Board list </h1>
    <p>{listOrLoad}</p>
  </div>
}