import React from 'react';
import translations from './translations';

export default function Loader() {
  return (
    <>
      <svg version="1.1" viewBox="0 0 100 100" enableBackground="new 0 0 100 100" className="loader">
        <circle fill="none" stroke="#fff" strokeWidth={4} strokeMiterlimit={10} cx={50} cy={50} r={48}/>
        <line fill="none" strokeLinecap="round" stroke="#fff" strokeWidth={4} strokeMiterlimit={10} x1={50} y1={50}
              x2={85} y2="50.5">
          <animateTransform attributeName="transform" dur="2s" type="rotate" from="0 50 50" to="360 50 50"
                            repeatCount="indefinite"/>
        </line>
        <line fill="none" strokeLinecap="round" stroke="#fff" strokeWidth={4} strokeMiterlimit={10} x1={50} y1={50}
              x2="49.5" y2={74}>
          <animateTransform attributeName="transform" dur="15s" type="rotate" from="0 50 50" to="360 50 50"
                            repeatCount="indefinite"/>
        </line>
      </svg>
      {translations.loading}
    </>
  );
}
