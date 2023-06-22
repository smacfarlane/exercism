#lang racket

(provide sum-of-squares square-of-sum difference)

(define (sum-of-squares number)
  (apply + (build-list (+ 1 number) (lambda (n) (* n n)))))

(define (square-of-sum number)
  (let ( [sum (apply + (build-list (+ 1 number) values))] )
       (* sum sum)
  )
)

(define (difference number)
  (- (square-of-sum number) (sum-of-squares number)))
