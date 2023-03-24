# this serves as a reference implementation for the game in python

import numpy as np
import time

n = 4
board = [' ' for x in range(n * n + 1)]


def markSquare(player, position):
    board[position] = player


def printBoard(board):
    print("\n")
    for i in range(1, n + 1):
        for x in range(1, n + 1):
            # mööööp
            # print (i,x, board[(i-1)*n+x])
            print(' ' + board[(i - 1) * n + x] + ' ', end='')
            if x < n:
                print('|', end='')
        print()
        if i < n:
            print(n * '----')


def isEnd(board):
    if isRow(board):
        return True

    if isCol(board):
        return True

    if isDiagonal(board):
        return True

    return False


def isDiagonal(board):
    # check for diagonal from top left to bottom right
    sol = 0

    for i in range(1, n + 1):
        if board[n * (i - 1) + i] == 'X':
            sol += 1

    if sol == n:
        return True

    # check for diagonal from top right to bottom left
    sol = 0

    for i in range(1, n + 1):
        if board[n * (i - 1) + (n - i + 1)] == 'X':
            sol += 1

    if sol == n:
        return True

    return False


def isRow(board):
    for ze in range(1, n + 1):
        sol = 0

        x = 1 + (ze - 1) * n
        for i in range(1, n + 1):

            if board[x] == 'X':
                sol += 1
            x += 1

        if sol == n:
            return True
    return False


def isCol(board):
    for sp in range(1, n + 1):
        sol = 0

        x = 1 + (sp - 1)
        for i in range(1, n + 1):

            if board[x] == 'X':
                sol += 1
            x += n

        if sol == n:
            return True
    return False


def isBoardFull(board):
    return (board.count(' ') == 1)


def playerMove():
    run = True
    while run:
        move = input('Bitte wähle eine Position für dein nächstes \'X\' (1-{}): '.format(str(n * n)))
        try:
            move = int(move)
            if 0 < move < (n * n + 1):
                if board[move] == ' ':
                    run = False
                    markSquare('X', move)
                else:
                    print('Sorry, diese Position ist schon besetzt!')
            else:
                print('Du musst bitte eine Position von 1 bis {} wählen!'.format(str(n * n)))
        except:
            print('Bitte gebe eine Zahl an!')


# zur Verfolgung der Komplexität
numNodes = 0


def compMove():
    possibleMoves = [x for x, letter in enumerate(board) if letter == ' ' and x != 0]
    print("Mögliche Züge für die KI = ", possibleMoves)

    # return random.choice(possibleMoves)
    bestScore = -1000000
    bestMove = 0

    boardCopy = board[:]
    for move in possibleMoves:
        boardCopy[move] = 'X'

        # Bewertung durch MiniMax-Algorithmus
        # score = minimax(boardCopy, 0, False)
        score = minimaxAlphaBeta(boardCopy, -1000000, 1000000, 0, False)
        print("Bewertung von Zug ", move, ": ", score)
        if score > bestScore:
            bestScore = score
            bestMove = move

        boardCopy[move] = ' '

    return bestMove


def minimax(currBoard, depth, isMaximizing):
    # terminal states
    if isEnd(currBoard) and isMaximizing:
        return 1
    elif isEnd(currBoard) and not isMaximizing:
        return -1

    # recursive minimax
    possibleMoves = [x for x, letter in enumerate(currBoard) if letter == ' ' and x != 0]
    if isMaximizing:

        # -1000 is like -infinity in this case
        bestScore = -1000000
        for move in possibleMoves:
            currBoard[move] = 'X'
            score = minimax(currBoard, depth + 1, False)
            currBoard[move] = ' '
            bestScore = np.maximum(score, bestScore)
        return bestScore

    else:
        bestScore = 1000000
        for move in possibleMoves:
            currBoard[move] = 'X'
            score = minimax(currBoard, depth + 1, True)
            currBoard[move] = ' '
            bestScore = np.minimum(score, bestScore)
        return bestScore


# rewrite with alpha beta pruning
def minimaxAlphaBeta(currBoard, alpha, beta, depth, isMaximizing):
    # terminal states
    if isEnd(currBoard) and isMaximizing:
        return (1)
    elif isEnd(currBoard) and not isMaximizing:
        return (-1)

    # recursive minimax
    possibleMoves = [x for x, letter in enumerate(currBoard) if letter == ' ' and x != 0]
    if isMaximizing:

        # -1000 is like -infinity in this case
        bestScore = -1000000
        for move in possibleMoves:
            currBoard[move] = 'X'
            score = minimaxAlphaBeta(currBoard, alpha, beta, depth + 1, False)
            currBoard[move] = ' '
            bestScore = np.maximum(score, bestScore)
            #print(score, bestScore)
            alpha = np.maximum(alpha, bestScore)
            if beta <= alpha:
                break
        return bestScore

    else:
        bestScore = 1000000
        for move in possibleMoves:
            currBoard[move] = 'X'
            score = minimaxAlphaBeta(currBoard, alpha, beta, depth + 1, True)
            currBoard[move] = ' '
            bestScore = np.minimum(score, bestScore)
            #print(score, bestScore)
            beta = np.minimum(beta, bestScore)
            if beta <= alpha:
                break
        return bestScore


def main():
    print('Wilkommen zu {} Verliert!'.format(n))
    printBoard(board)

    while not (isBoardFull(board)):
        if not (isEnd(board)):
            playerMove()
            printBoard(board)
        else:
            print('Du hast gewonnen! Gute Arbeit!')
            break

        if not (isEnd(board)):
            start = time.time()
            move = compMove()
            end = time.time()
            print('Das dauerte: {} s'.format(end - start))

            markSquare('X', move)
            print('Die künstliche "Intelligenz" setzt \'X\' auf Position', move, ':')
            printBoard(board)
        else:
            print('Sorry, die KI hat dieses mal gewonnen!')
            break


while True:
    answer = input('Willst du spielen? (y/n)')
    if answer.lower() == 'y' or answer.lower == 'yes':
        board = [' ' for x in range(n * n + 1)]
        print('---------------------------------------')
        main()
    else:
        break
