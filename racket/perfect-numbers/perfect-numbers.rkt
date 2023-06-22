#lang racket
(require math)
(provide classify)

(define (classify number)
  (cond
   [(= (sum-of-factors number) number) 'perfect] 
   [(> (sum-of-factors number) number) 'abundant] 
   [(< (sum-of-factors number) number)  'deficient] 
  )
)

(define (sum-of-factors number)
  (apply + (remove number (divisors number)))
)