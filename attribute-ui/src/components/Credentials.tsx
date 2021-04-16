import React, { Fragment } from 'react';
import Layout from './Layout';
import translations from './translations';
import { Check } from './icons';

export default function Credentials({ data }) {
  return (
    <Layout>
      <div className="reason">
        <h4>{translations.purpose}</h4>
        <p>
          {translations[data.purpose] || data.purpose}
        </p>
      </div>
      <div className="attributes">
        <div>
          <h4>
            <Check/>
            {translations.attributes}
          </h4>
          <dl>
            {Object.entries(data.auth_result.attributes).map(([key, value]) => (
              <Fragment key={key}>
                <dt>{translations[key] || key}</dt>
                <dd>{value}</dd>
              </Fragment>
            ))}
          </dl>
        </div>
      </div>
    </Layout>
  );
}
