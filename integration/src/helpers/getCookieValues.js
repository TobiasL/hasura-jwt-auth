const getCookieValues = (cookieString) => {
  const cookiesValues = cookieString.split(';').map((str) => str.trim())

  const refresh = cookiesValues.find((val) => val.startsWith('refresh=')).replace('refresh=', '')
  const jwtToken = cookiesValues.find((val) => val.startsWith('jwt=')).replace('jwt=', '')
  const jwtExpiry = cookiesValues.find((val) => val.startsWith('jwt_expiry=')).replace('jwt_expiry=', '')
  const refreshExpiry = cookiesValues.find((val) => val.startsWith('refresh_expiry=')).replace('refresh_expiry=', '')

  return {
    refresh,
    jwtToken: jwtToken.replace('Bearer ', ''),
    jwtExpiry: parseInt(jwtExpiry, 10),
    refreshExpiry: parseInt(refreshExpiry, 10),
  }
}

module.exports = getCookieValues
