;;

;;(load-shared-object "/Users/d/Work/my/tensorflow/bazel-bin/tensorflow/tools/lib_package/tmp/lib/libtensorflow.so")

;;(define tf_Version
;;    (foreign-procedure "TF_Version" ()
;;      void*))

;;(define tf_NewStatus
;;    (foreign-procedure "TF_NewStatus" ()
;;      void*))

;;(define tf_DeleteStatus
;;    (foreign-procedure "TF_DeleteStatus" (void*) 
;;      void))

;;(define tf_SetStatus
;;    (foreign-procedure "TF_SetStatus" (void* int string)
;;      void))

;;(define tf_GetCode
;;    (foreign-procedure "TF_GetCode" (void*)
;;      int))

;;(define tf_Message
;;    (foreign-procedure "TF_Message" (void*)
;;      string))


;;(let ([status (tf_NewStatus)]) 
;;    (tf_SetStatus status 3932 "haha haah iii Some error")
;;    (display (tf_GetCode status)) (newline)
;;    (display (tf_Message status)) (newline))


;;





(define-syntax generator
   (syntax-rules ()
     (
       (generator (yieldfunc) body ...)
       (let (
             (placeholder #f)  ;;placeholder in this function
             (return-proc #f)  ;;how to get out
             (finished #f))    ;;whether or not it is finished
         ;;this is the generator entrance
         (lambda ()
           (call-with-current-continuation
             (lambda (tmp-return-proc)
               ;;save the way out in "return-proc"
               (set! return-proc tmp-return-proc)
               (let (
                     ;;"yieldfunc" resets the placeholder and returns the value
                     (yieldfunc
                       (lambda (x)
                         (call-with-current-continuation
                           (lambda (current-placeholder)
                             ;;new placeholder
                             (set! placeholder current-placeholder)
                             ;;return value
                             (return-proc x))))))
 
                 ;;If the generator is done, return a special value
                 (if finished
                     'generator-finished
 
                     ;;If this is the first time, there will not be a
                     ;;placeholder established, so we just run the body.
                     ;;If there is a placeholder, we resume to that point
                     (if placeholder
                       (placeholder 'unspecified)
                       (let (
                             (final-value (begin body ...)))
                         (set! finished #t)
                         (return-proc final-value))))))))))))
 
(define sequence-generator
  ;;Initial parameters
  (lambda (start end increment)
    ;;"yield" will be used to generate a single value
    (generator (yield)
      ;;Main function body
      (let loop ((curval start))
         (if (eqv? curval end)
             curval
             (begin
                ;;yield the value
                (yield curval)
                ;;continue on
                (loop (+ curval increment))))))))
 
;;Example usage
(define my-sequence (sequence-generator 1 3 1))
(display (my-sequence))(newline)
(display (my-sequence))(newline)
(display (my-sequence))(newline)
(display (my-sequence))(newline)



;; ---------
;; url: https://wiki.c2.com/?SchemeCoroutineExample

(define (coroutine routine)
   (let ((current routine)
         (status 'suspended))
     (lambda args
       (cond ((null? args) 
              (if (eq? status 'dead)
                  (display 'dead-coroutine)
                  (let ((continuation-and-value
                         (call/cc (lambda (return)
                                    (let ((returner
                                           (lambda (value)
                                             (call/cc (lambda (next)
                                                        (return (cons next value)))))))
                                      (current returner)
                                      (set! status 'dead))))))
                    (if (pair? continuation-and-value)
                        (begin (set! current (car continuation-and-value))
                               (cdr continuation-and-value))
                        continuation-and-value))))
             ((eq? (car args) 'status?) status)
             ((eq? (car args) 'dead?) (eq? status 'dead))
             ((eq? (car args) 'alive?) (not (eq? status 'dead)))
             ((eq? (car args) 'kill!) (set! status 'dead))
             (true nil)))))


(define test-coroutine-1
       (coroutine (lambda (yield)
                    (display "HELLO!") (newline)
                    (yield 1)
                    (display "WORLD!") (newline)
                    (yield 2)
                    (display "SORRY, I'M OUT") (newline))))


(define test-coroutine-2
	(coroutine (lambda (yield)
			(let ([x 0])
				(set! x 10)
				(display x) (newline)
				(yield 1)
				(set! x (+ x 10))
				(display x) (newline)
				(yield 2)))))




;; http://lambda-the-ultimate.org/
;; http://www.newsmth.net/bbsanc.php?path=%2Fgroups%2Fcomp.faq%2FFuncProgram%2Fprogramming%2FM.1083050874.R0
;; http://www.madore.org/~david/computers/callcc.html


;; SCMUTILS Reference Manual: https://groups.csail.mit.edu/mac/users/gjs/6946/refman.txt
;; Procedural approach to classical mechanics: http://fmnt.info/blog/20180228_sicm/index.html#sec-5
;; https://wingolog.org/archives/2013/01/07/an-opinionated-guide-to-scheme-implementations
;; https://twistedmatrix.com/users/glyph/rant/extendit.html

