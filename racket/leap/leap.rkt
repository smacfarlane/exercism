#lang racket

(provide leap-year?)

(define (leap-year? year)
  (and (divisible-by-4? year)
       (or (not (divisible-by-100? year))
           (divisible-by-400? year)
       )
  )
)

(define (divisible-by? dividend divisor)
  (= 0 (remainder dividend divisor))
)

(define (divisible-by-4? number)
  (divisible-by? number 4)
)
(define (divisible-by-100? number)
  (divisible-by? number 100)
)
(define (divisible-by-400? number)
  (divisible-by? number 400)
)
