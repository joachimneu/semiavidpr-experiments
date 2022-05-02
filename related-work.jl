#! /usr/bin/env julia

n = 1024
F = 22e6

κ_hash = 32   # Bytes
κ_commit = 48   # Bytes
# κ_sig = 32   # not counting signatures (all VID) and blockchain interaction (ACeD)


ln(x) = log(x)

function repetition(n, t, F)
    k = n - 2*t

    # client sends full file to all storage nodes
    communication = n * F
    # all storage nodes store full file
    storage = n * F

    return (communication, storage)
end

function avid(n, t, F)
    k = n - 2*t

    # client sends to each storage node: chunk, hash vector
    # storage nodes echo their chunks and hash vectors to each other
    communication = (F/k + n*κ_hash) * (n + n^2)    
    # chunk and hash vector are stored at each storage node
    storage = n * (F/k + n*κ_hash)

    return (communication, storage)
end

function avid_fp(n, t, F)
    k = n - 2*t

    # (assuming homomorphic fingerprints are only of size of a hash)
    # msg = chunk + hash vector + fingerprint vector
    # client sends to each storage node one msg,
    # storage nodes echo to each other the hash and fingerprint vectors
    communication = n * (F/k + (n+k)*κ_hash) + n^2 * (n+k)*κ_hash
    # nodes store their messages (for retrieving client)
    storage = n * (F/k + (n+k)*κ_hash)

    return (communication, storage)
end

function avid_m(n, t, F)
    k = n - 2*t

    # client sends to storage nodes their chunk + opening wrt Merkle root
    # nodes echo to each other the Merkle root
    communication = n*(F/k + (1 + log2(n))*κ_hash) + n^2 * κ_hash
    # nodes store their chunk + opening (for retrieving client)
    storage = n*(F/k + (1 + log2(n))*κ_hash)

    return (communication, storage)
end

function aced(n, t, F; c=48e3)
    t_ = 16
    r = 0.25
    q = 8
    eta = 0.875
    lambda = (1 - 2*t/n) / ln(1 / (1-eta))
    d = 8

    # source: https://arxiv.org/abs/2011.00102v2
    # Theorem 2, 2.) + eqn (5)
    communication = n * (  t_ * κ_hash + F/(n*r*lambda) + (2*q - 1)*F*κ_hash/(n*r*c*lambda) * log(F / (c * t_ * r)) / log(q*r)  )
    # eqn (5)
    storage = communication
    # eqn (6)
    proof = (d-1)*c + d*κ_hash*(q-1) *  log(F / (c * t_ * r)) / log(q*r)

    return (communication, storage, proof)
end

function semiavidpr(n, t, F)
    k = n - 2*t

    # communication = n * (F/k + k*κ_commit + κ_sig)   # (ignoring signatures)
    # client sends to each storage node their chunk + column commitments
    communication = n * (F/k + k*κ_commit)
    # nodes store their chunks + column commitments (for retrieving client)
    storage = n * (F/k + k*κ_commit)

    return (communication, storage)
end


t_049 = round(0.49*n)
t_033 = round(0.33*n)

@show (t_049, t_033)

@show repetition(n, t_049, F)
@show avid(n, t_033, F)
@show avid_fp(n, t_033, F)
@show avid_m(n, t_033, F)
@show aced(n, t_033, F; c=40e3)
@show aced(n, t_049, F; c=40e3)
@show semiavidpr(n, t_033, F)
@show semiavidpr(n, t_049, F)


for c in 8e3:8e3:96e3
    @show c
    (C, S, P) = aced(n, t_033, F; c=c)
    @show (C, S, P)
end


println("c proof storage")
for c in 8e3:8e3:96e3
    (C, S, P) = aced(n, t_033, F; c=c)
    println("$c $P $S")
end

