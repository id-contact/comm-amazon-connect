import React from 'react';
import Layout from './Layout';
import translations from './translations';

export default function Error({ error }) {
  return (
    <Layout>
      <div className="error">
        <h4>{translations.error}</h4>
        <p>{error}</p>
        <a
          href="#"
          onClick={(e) => {
            e.preventDefault();
            window.location.reload(true);
          }}
        >
          {translations.refresh}
        </a>
      </div>
    </Layout>
  );
}
