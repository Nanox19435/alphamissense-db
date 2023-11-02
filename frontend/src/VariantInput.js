import React, { useState } from 'react';

const VariantInput = ({id}) => {
  const [inputText, setInputText] = useState('');
  const [parsedText, setParsedText] = useState(null);
  const [receivedText, setReceivedText] = useState('');

  const handleInputChange = (e) => {
    setInputText(e.target.value);
    parseInput(e.target.value);
    setReceivedText('')
  };

  const parseInput = (value) => {
    const variantRegEx = /[ARNDCEQGHILKMFPSTWYV]\d+[ARNDCEQGHILKMFPSTWYV]/;

    if (variantRegEx.test(value)) {
      setParsedText(value); // Store parsed text if successful
    } else {
      setParsedText(null); // Set parsedText to null on parsing failure
    }
  };

  const handleParseClick = () => {
    if (parsedText) {
      const apiUrl = `http://127.0.0.1:8000/variants/${id}/${parsedText}`;
  
      fetch(apiUrl)
        .then((response) => {
          if (!response.ok) {
            throw new Error('HTTP request failed');
          }
          return response.text();
        })
        .then((data) => {
          console.log('Received data:', data);
          setReceivedText(data)
        })
        .catch((error) => {
          console.error('Error fetching data:', error);
        });
    }
  };

  const inputStyle = {
    color: parsedText === null ? 'red' : 'black',
    width: '50%',
  };

  return (
    <div style={{ textAlign: 'center' }}>
      <input
        type="text"
        value={inputText}
        onChange={handleInputChange}
        placeholder={parsedText ? parsedText : 'Inserte la variante que desea buscar'}
        style={inputStyle}
      />
      {parsedText && <button onClick={handleParseClick}>Parse</button>}
      {parsedText && <p style={{ color: parsedText === null ? 'red' : 'black' }}></p>}
      <div>
        {receivedText && <p>{receivedText}</p>}
      </div>
    </div>
  );
};

export default VariantInput;
