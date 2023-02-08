export async function POST(data, route)
{
     const res = await fetch('http://localhost:3000/' + route, {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(data)
  });
  const content = await res.json();

  return content;
}
export async function GET(route)
{
  const res = await fetch('http://localhost:3000/' + route)
  const content = await res.json();
  return content;
}
