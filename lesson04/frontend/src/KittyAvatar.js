import React from 'react';

const KittyAvatar = props => {
  const outerStyle = { height: '160px', position: 'relative', width: '50%' };
  const innerStyle = {
    height: '150px',
    position: 'absolute',
    top: '3%',
    left: '50%',
  };
  const { asset } = props;

  if (!asset) return null;

  const cat = `${process.env.PUBLIC_URL}/assets/KittyAvatar/${asset}.png`
  return (
    <div style={outerStyle}>
      <img alt="cat" src={cat} style={innerStyle} />
    </div>
  );
};

export default KittyAvatar;
