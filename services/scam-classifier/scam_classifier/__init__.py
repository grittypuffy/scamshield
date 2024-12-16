import pandas as pd
import numpy as np
import json
from sklearn.model_selection import train_test_split
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics import classification_report, accuracy_score, confusion_matrix
from sklearn.ensemble import GradientBoostingClassifier
from sklearn.pipeline import make_pipeline
from sklearn.impute import SimpleImputer
import pickle

def load_data(json_file):
    with open(json_file, 'r') as f:
        data = json.load(f)

    df = pd.DataFrame(data)
        
    return df

df = load_data('../../datasets/forex_scams_dataset_v1.json')

df.dropna(subset=['text'], inplace=True)

df['category'] = df['category'].fillna('UNKNOWN')

# Text is set to features and scam is used as a label here
X = df['text']
y = df['scam']

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.25, random_state=40)

tfidf = TfidfVectorizer(stop_words='english', max_features=5000)

X_train_tfidf = tfidf.fit_transform(X_train)
X_test_tfidf = tfidf.transform(X_test)

gb_model = GradientBoostingClassifier(n_estimators=100, learning_rate=0.5, max_depth=3, random_state=40)

gb_model.fit(X_train_tfidf, y_train)

y_pred = gb_model.predict(X_test_tfidf)

print("Accuracy: ", accuracy_score(y_test, y_pred))
print("Confusion Matrix: \n", confusion_matrix(y_test, y_pred))
print("Classification Report: \n", classification_report(y_test, y_pred))

with open('gradient_boosting_scam_model.pkl', 'wb') as model_file:
    pickle.dump(gb_model, model_file)

with open('tfidf_vectorizer.pkl', 'wb') as tfidf_file:
    pickle.dump(tfidf, tfidf_file)
