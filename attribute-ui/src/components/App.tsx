import React, { useState, useEffect } from 'react';
import axios from 'axios';
import Loader from './Loader';
import Layout from './Layout';
import translations from './translations';
import Credentials from './Credentials';
import Error from './Error';

declare var SERVER_URL: string;

export default function App() {
  const [data, setData] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    const sessionid = decodeURIComponent(location.search).substr(1);
    axios
      .get(`${SERVER_URL}/session_info/${sessionid}`).then((response) => {
        setData(response.data);
      })
      .catch((e) => {
        if (e.response?.data?.error?.description) {
          setError(e.response.data.error.description);
        } else {
          setError(translations.unknown_error);
        }
      });
  }, [location.search]);

  if (error) {
    return (
      <Error error={error} />
    );
  }

  if (!data?.auth_result || !data.purpose) {
    return (
      <Layout>
        <Loader/>
      </Layout>
    );
  }

  return (
    <Credentials data={data} />
  );
}
