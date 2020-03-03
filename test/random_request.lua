-- example dynamic request script which demonstrates changing
-- the request path and a header for each request
-------------------------------------------------------------
-- source https://github.com/wg/wrk/blob/master/scripts/counter.lua


request = function()
   path = "/lines/" .. math.random(262140)
   return wrk.format(nil, path)
end