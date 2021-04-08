import React, {useState, useEffect} from 'react';
import axios from 'axios';

declare var server: string;

export default function App() {
  const [data, setData] = useState(null);

  useEffect(() => {
    let sessionid = decodeURIComponent(location.search).substr(1);
    axios.get(`${server}/session_info/${sessionid}`).then((response) => {
      setData(response.data);
    });
  }, [location.search]);

  return (
    <h1>{data?.purpose ?? 'loading'}</h1>
  );
}
